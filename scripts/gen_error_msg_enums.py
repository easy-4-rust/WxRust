#!/usr/bin/env python3
"""错误码枚举代码生成器：从 WxJava Java 枚举源码提取错误码→中文映射，生成 Rust。

输入：weixin-java-common/src/main/java/me/chanjar/weixin/common/error/Wx*ErrorMsgEnum.java
输出：crates/wx-rust-common/src/error/<wx_xxx>_error_msg_enum.rs

解析规则：每行形如 `  CODE_40001(40001, "中文信息"),` —— 提取数字与中文。
重复错误码（Java 枚举中允许重复 code 常量）取第一条。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
JAVA_DIR = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-common/src/main/java/me/chanjar/weixin/common/error"
RUST_DIR = os.path.join(BASE, "crates", "wx-rust-common", "src", "error")

ENTRY_RE = re.compile(r'^\s*CODE_(\w+)\((-?\d+),\s*"((?:[^"\\]|\\.)*)"\)')

MODULES = {
    "WxMpErrorMsgEnum": ("wx_mp", "微信公众号"),
    "WxCpErrorMsgEnum": ("wx_cp", "企业微信"),
    "WxMaErrorMsgEnum": ("wx_ma", "微信小程序"),
    "WxOpenErrorMsgEnum": ("wx_open", "微信开放平台"),
    "WxChannelErrorMsgEnum": ("wx_channel", "微信视频号"),
}


def unescape(s: str) -> str:
    """还原 Java 字符串字面量；并转义 Rust 字符串中需要转义的反斜杠与引号。"""
    s = s.replace("\\\"", '"').replace("\\\\", "\\")
    # Rust 字符串字面量中反斜杠与双引号必须转义
    return s.replace("\\", "\\\\").replace('"', '\\"')


def gen(java_name, rust_prefix, display_name):
    src = os.path.join(JAVA_DIR, java_name + ".java")
    entries = []
    seen = set()
    with open(src, encoding="utf-8") as f:
        for line in f:
            m = ENTRY_RE.match(line)
            if m:
                _, code_str, msg = m.groups()
                code = int(code_str)
                if code in seen:
                    continue
                seen.add(code)
                entries.append((code, unescape(msg)))
    if not entries:
        print(f"[warn] {java_name}: 未解析到条目")
        return None

    # 按错误码排序，保证确定性输出
    entries.sort(key=lambda x: x[0])

    out = []
    out.append(f"//! {display_name}全局返回码错误信息表。")
    out.append(f"//!")
    out.append(f"//! 对应 Java `me.chanjar.weixin.common.error.{java_name}`，")
    out.append(f"//! 由 `scripts/gen_error_msg_enums.py` 从 Java 源码自动生成（{len(entries)} 条错误码）。")
    out.append("")
    out.append(f"/// 按错误码查找{display_name}中文错误信息。")
    out.append("///")
    out.append("/// # 参数")
    out.append("/// - `code`：微信错误码")
    out.append("///")
    out.append("/// # 返回")
    out.append("/// 错误码对应的中文信息；未收录时返回 `None`。")
    out.append(f"pub fn find_msg_by_code(code: i32) -> Option<&'static str> {{")
    out.append("    match code {")
    for code, msg in entries:
        out.append(f'        {code} => Some("{msg}"),')
    out.append("        _ => None,")
    out.append("    }")
    out.append("}")
    out.append("")
    return "\n".join(out)


def main():
    os.makedirs(RUST_DIR, exist_ok=True)
    total = 0
    for java_name, (rust_prefix, display_name) in MODULES.items():
        body = gen(java_name, rust_prefix, display_name)
        if body is None:
            continue
        dst = os.path.join(RUST_DIR, f"{rust_prefix}_error_msg_enum.rs")
        with open(dst, "w", encoding="utf-8") as f:
            f.write(body)
        n = body.count("Some(")
        total += n
        print(f"[ok] {java_name} -> {os.path.relpath(dst, BASE)}（{n} 条）")
    print(f"合计 {total} 条错误码映射")


if __name__ == "__main__":
    sys.exit(main())
