#!/usr/bin/env python3
"""WxConsts 常量生成器：从 Java WxConsts.java 提取全部常量，生成 Rust。

规则：
- 顶层常量 -> `pub const`
- @UtilityClass 内部类 -> Rust 独立常量组（同文件，`pub mod` 或前缀常量）
- ACCESS_TOKEN_ERROR_CODES = Arrays.asList(CODE_40001...) -> 用错误码枚举的数值展开
- 字符串常量原样保留；含 %s 的模板常量保留
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-common/src/main/java/me/chanjar/weixin/common/api/WxConsts.java"
RUST_DIR = os.path.join(BASE, "crates", "wx-rust-common", "src", "api")

# 从错误码枚举提取 CODE_xxx -> 数字
CODE_NUMS = {}
with open("/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-common/src/main/java/me/chanjar/weixin/common/error/WxMpErrorMsgEnum.java", encoding="utf-8") as f:
    for line in f:
        m = re.match(r'\s*CODE_(\w+)\((-?\d+),', line)
        if m:
            CODE_NUMS["CODE_" + m.group(1)] = int(m.group(2))

CONST_RE = re.compile(r'^\s*public static final (\w+) (\w+) = (.+);')
CLASS_RE = re.compile(r'^\s*public static class (\w+) \{')


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    return s.upper()


def parse():
    """返回 (top_consts, inner_groups)。inner_groups: {name: [(const_type, name, value)]}"""
    top = []
    inner = {}
    cur = None
    with open(JAVA, encoding="utf-8") as f:
        for line in f:
            cm = CLASS_RE.match(line)
            if cm:
                cur = cm.group(1)
                inner[cur] = []
                continue
            cm2 = re.match(r'^\s*public static class (\w+)', line)
            if cm2 and not line.rstrip().endswith("{"):
                cur = cm2.group(1)
                inner.setdefault(cur, [])
                continue
            m = CONST_RE.match(line)
            if m:
                ctype, name, value = m.groups()
                if cur:
                    inner[cur].append((ctype, name, value))
                else:
                    top.append((ctype, name, value))
    return top, inner


def rust_const(ctype, name, value):
    """把 Java 常量表达式转为 Rust 常量"""
    v = value.strip()
    # CODE_xxx.getCode() -> 数字
    m = re.match(r"CODE_(\w+)\.getCode\(\)", v)
    if m:
        code = CODE_NUMS.get("CODE_" + m.group(1))
        if code is not None:
            return f'    pub const {snake(name)}: i32 = {code};'
    # 字符串常量
    if v.startswith('"'):
        return f'    pub const {snake(name)}: &str = {v};'
    # 数字
    if re.fullmatch(r"-?\d+", v):
        return f'    pub const {snake(name)}: {ctype_map(ctype)} = {v};'
    # List 常量（ACCESS_TOKEN_ERROR_CODES 等）—— 跳过，改用函数
    if v.startswith("Arrays.asList"):
        codes = re.findall(r"CODE_(\w+)\.getCode\(\)", v)
        nums = [CODE_NUMS.get("CODE_" + c) for c in codes if CODE_NUMS.get("CODE_" + c) is not None]
        return f'    pub const {snake(name)}: &[i32] = &[{", ".join(str(n) for n in nums)}];'
    # 布尔
    if v in ("true", "false"):
        return f'    pub const {snake(name)}: bool = {v};'
    # 其它复杂表达式 -> 注释保留
    return f'    // {name} = {v}（复杂表达式，需人工核对）'


def ctype_map(t: str) -> str:
    return {"String": "&str", "int": "i32", "long": "i64", "boolean": "bool"}.get(t, "&str")


def main():
    top, inner = parse()
    out = []
    out.append("//! 微信常量集合。")
    out.append("//!")
    out.append("//! 对应 Java `me.chanjar.weixin.common.api.WxConsts`，")
    out.append("//! 由 `scripts/gen_wx_consts.py` 从 Java 源码自动生成。")
    out.append("")
    out.append("/// access_token 相关错误代码。")
    out.append("///")
    out.append("/// 发生以下情况时尝试刷新 access_token：")
    out.append("/// - 40001：获取 access_token 时 AppSecret 错误，或者 access_token 无效")
    out.append("/// - 40014：不合法的 access_token")
    out.append("/// - 42001：access_token 超时")
    out.append("pub const ACCESS_TOKEN_ERROR_CODES: &[i32] = &[40001, 40014, 42001];")
    out.append("")
    out.append("/// 微信接口返回的参数 `errcode`。")
    out.append("pub const ERR_CODE: &str = \"errcode\";")
    out.append("")
    for ctype, name, value in top:
        if name in ("ACCESS_TOKEN_ERROR_CODES", "ERR_CODE"):
            continue
        c = rust_const(ctype, name, value)
        if c and not c.startswith("    // "):
            out.append(c)
    out.append("")

    for gname, consts in inner.items():
        if not consts:
            continue
        out.append(f"/// 微信消息/参数类型常量组（对应 Java `WxConsts.{gname}`）。")
        out.append(f"pub mod {snake(gname).lower()} {{")
        out.append("")
        for ctype, name, value in consts:
            c = rust_const(ctype, name, value)
            if c:
                out.append(c)
        out.append("}")
        out.append("")

    dst = os.path.join(RUST_DIR, "wx_consts.rs")
    with open(dst, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"[ok] -> {os.path.relpath(dst, BASE)}（顶层 {len(top)} 常量，{len(inner)} 内部组）")


if __name__ == "__main__":
    sys.exit(main())
