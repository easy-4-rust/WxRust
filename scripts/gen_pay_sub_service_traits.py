#!/usr/bin/env python3
"""Wave 5 P5：由 Java 子服务接口生成 Rust trait。

读取 WxJava `weixin-java-pay .../service/*Service.java`（排除 WxPayService），
为每个接口生成 `crates/wx-rust-pay/src/api/<snake>.rs`：
- 方法全部 async（本 crate 门面为 async 体系，Java 同步方法语义不变）；
- String 参数 -> &str、int -> i32、long -> i64、boolean -> bool；
- bean 参数 -> `&T`（实现体 clone 后签名装配，同门面惯例）；
- 枚举参数（Copy）按值传递；
- Java 泛型 `<T> T` 返回值 -> `serde_json::Value`（类型擦除，同门面 create_order_v3）；
- File/InputStream 媒体参数 -> `(file_name: &str, file_data: &[u8])`（ADAPTED，
  重载合并为单方法；同名不同元数重载改名 *_with_* 并注记）。

输出文件仅供人工校对后提交；方法体由 P5 手工镜像 Java impl。
"""
import os
import re

JAVA_SVC = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-pay/src/main/java/com/github/binarywang/wxpay/service"
OUT_DIR = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust/crates/wx-rust-pay/src/api"

VALUE_TYPES = {
    "TradeTypeEnum", "SpAccountTypeEnum", "FundBillTypeEnum", "GlobalTradeTypeEnum",
    "StockTypeEnum", "JumpTargetEnum", "BackgroundColorEnum",
}


def snake(name: str) -> str:
    s = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name)
    s = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])", "_", s)
    return s.lower()


def rust_type(t: str) -> str:
    t = t.strip()
    if t in ("String",):
        return "&str"
    if t == "int":
        return "i32"
    if t == "Integer":
        return "i32"
    if t == "long":
        return "i64"
    if t == "Long":
        return "i64"
    if t == "boolean":
        return "bool"
    if t == "Boolean":
        return "bool"
    if t == "Double":
        return "f64"
    if t == "BigDecimal":
        return "i64"
    if t == "void" or t == "Void":
        return "()"
    if t == "InputStream":
        return "Vec<u8>"
    if t == "<T> T":
        return "serde_json::Value"
    if t in VALUE_TYPES:
        return t
    # 其余按 bean/枚举类型名原样保留（参数侧由调用方决定引用）
    return t.strip()


def _split_top(s: str) -> list:
    parts, depth, cur = [], 0, []
    for ch in s:
        if ch in "<(":
            depth += 1
        elif ch in ">)":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append("".join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    if cur:
        parts.append("".join(cur).strip())
    return parts


def parse_params(params_src: str):
    """返回 (rust_params, adapted_flags)。"""
    if not params_src.strip():
        return [], []
    parts = _split_top(params_src)
    out = []
    flags = []
    for p in parts:
        p = re.sub(r"@\w+", "", p).strip()  # 去除 @NonNull 等注解
        m = re.match(r"([\w\.<>\[\]]+)\s+(\w+)$", p.strip())
        if not m:
            raise RuntimeError(f"无法解析参数: {p}")
        jtype, name = m.group(1), m.group(2)
        rname = snake(name)
        if jtype in ("File", "InputStream"):
            # Java 文件/流参数：ADAPTED 为 (文件名, 字节)
            out.append((rname, "&str", "media"))
            flags.append("media")
            continue
        rt = rust_type(jtype)
        if rt not in VALUE_TYPES and rt != "&str" and not rt.isdigit() and rt not in (
            "i32", "i64", "bool", "f64",
        ):
            out.append((rname, f"&{rt}", None))
        else:
            out.append((rname, rt, None))
    return out, flags


def gen_one(java_file: str) -> str:
    src = open(java_file, encoding="utf-8").read()
    body = re.sub(r"/\*\*.*?\*/", "", src, flags=re.S)
    body = re.sub(r"//[^\n]*", "", body)
    iface_name = os.path.basename(java_file)[:-5]
    m = re.search(r"interface\s+" + iface_name + r"\b", body)
    assert m, f"interface not found in {java_file}"
    body = body[m.end():]
    body = body[: body.rfind("}")]

    methods = []
    for decl in re.finditer(
        r"(?:public\s+|default\s+)?([\w<>\.\[\],\s]+?)\s+(\w+)\s*\(([^)]*)\)\s*(?:throws\s+[\w\.,\s]+)?;",
        body,
        re.S,
    ):
        ret_raw, name, params_raw = decl.group(1).strip(), decl.group(2), decl.group(3)
        if name in ("getClass", "hashCode", "toString"):
            continue
        params, flags = parse_params(params_raw)
        methods.append((ret_raw, name, params, flags))

    # 收集 javadoc 摘要（方法名 -> 首行 doc）
    docs = {}
    for mdoc in re.finditer(r"/\*\*\s*\n(.*?)\*/", src, re.S):
        head = mdoc.group(1)
        lines = [ln.strip().lstrip("*").strip() for ln in head.split("\n")]
        lines = [ln for ln in lines if ln and not ln.startswith("@")]
        after = src[mdoc.end():]
        nm = re.search(r"(?:public\s+|default\s+)?[\w<>\.\[\],\s]+?\s+(\w+)\s*\(", after)
        if nm:
            txt = " ".join(lines)
            txt = re.sub(r"<[^>]+>", "", txt).replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
            txt = re.sub(r"\s+", " ", txt).strip()
            docs.setdefault(nm.group(1), txt[:100])

    # 重载处理：相同 (rust 名, 参数列表) 去重合并；同名不同元数改名
    seen = {}
    out_methods = []
    for ret_raw, name, params, flags in methods:
        rname = snake(name)
        sig = (rname, tuple((p[0], p[1]) for p in params))
        if sig in seen:
            continue  # 合并重载（File/InputStream 变体收敛为同一签名）
        if rname in seen and seen[rname] != sig:
            # 同名不同元数：第二个及以后改名 *_with_*
            rname = rname + "_with_account_type"
        seen[rname] = sig
        out_methods.append((ret_raw, name, params, flags, rname))

    lines = []
    lines.append(f"//! 对应 Java `com.github.binarywang.wxpay.service.{iface_name}`。")
    lines.append("//!")
    lines.append("//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成")
    lines.append("//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java")
    lines.append("//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED")
    lines.append("//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。")
    lines.append("")
    lines.append("use async_trait::async_trait;")
    lines.append("use wx_rust_common::error::WxErrorException;")
    lines.append("")
    lines.append(f"/// {iface_name}（对应 Java `{iface_name}`）。")
    lines.append("#[async_trait]")
    lines.append(f"pub trait {iface_name}: Send + Sync {{")
    for ret_raw, name, params, flags, rname in out_methods:
        rtype = rust_type(ret_raw)
        doc = docs.get(name, "")
        if doc:
            lines.append(f"    /// {doc}")
        if "media" in flags:
            lines.append("    /// `ADAPTED`：Java `File`/`InputStream` 媒体参数以 `(文件名, 字节)` 表达。")
        if ret_raw.strip() == "<T> T":
            lines.append("    /// `ADAPTED`：Java 泛型 `<T> T` 返回值以 `serde_json::Value` 类型擦除。")
        plist = []
        for rn, rt, _fl in params:
            plist.append(f"{rn}: {rt}")
        ret = "Result<(), WxErrorException>" if rtype == "()" else f"Result<{rtype}, WxErrorException>"
        if plist:
            lines.append(f"    async fn {rname}(&self, {', '.join(plist)}) -> {ret};")
        else:
            lines.append(f"    async fn {rname}(&self) -> {ret};")
        lines.append("")
    lines.append("}")
    return "\n".join(lines)


def main():
    for f in sorted(os.listdir(JAVA_SVC)):
        if not f.endswith("Service.java") or f == "WxPayService.java":
            continue
        name = f[:-5]
        out = os.path.join(OUT_DIR, snake(name) + ".rs")
        content = gen_one(os.path.join(JAVA_SVC, f))
        open(out, "w", encoding="utf-8").write(content)
        print("wrote", out, len(content.splitlines()), "lines")


if __name__ == "__main__":
    main()
