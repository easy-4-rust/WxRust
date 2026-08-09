#!/usr/bin/env python3
"""Wave 5 P5：由 Java bean 子包枚举生成 Rust 枚举文件。

读取 WxJava `weixin-java-pay .../bean/{applyconfirm,applyment,ecommerce,marketing,
mipay,payscore}/enums/*.java`，为每个枚举生成
`crates/wx-rust-pay/src/bean/<pkg>/enums/<snake>.rs`：
- 变体名 = Java 常量名（SCREAMING_SNAKE_CASE 原样保留，serde 序列化值即常量名，
  对应 Java `name()` / `@SerializedName`）；
- 带构造参数的枚举（如 BackgroundColorEnum(value, code)）生成与 Java 字段
  同名的 getter（对应 Lombok @Getter）；
- 文件级 `#![allow(non_camel_case_types)]` 声明常量名变体。

并在各 `<pkg>/mod.rs` 注册 `pub mod enums;`（幂等）。
"""
import os
import re

BEAN = "/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-pay/src/main/java/com/github/binarywang/wxpay/bean"
OUT = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust/crates/wx-rust-pay/src/bean"
PACKAGES = ["applyconfirm", "applyment", "ecommerce", "marketing", "mipay", "payscore"]


def snake(name: str) -> str:
    s = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name)
    s = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])", "_", s)
    return s.lower()


def gen_one(path: str, pkg: str) -> str:
    src = open(path, encoding="utf-8").read()
    name = os.path.basename(path)[:-5]
    body = re.sub(r"/\*\*.*?\*/", "", src, flags=re.S)
    body = re.sub(r"//[^\n]*", "", body)
    m = re.search(r"enum\s+" + name + r"\b(.*?)\}", body, re.S)
    assert m, f"enum body not found: {path}"
    inner = m.group(1)

    # 变体：NAME、NAME,、NAME("arg1"[, "arg2"])、NAME; 等行
    variants = []
    for line in inner.splitlines():
        line = line.strip().rstrip(",").rstrip(";").strip()
        if not line:
            continue
        vm = re.match(r"(\w+)\s*(?:\((.*)\))?$", line)
        if vm and vm.group(1) not in variants:
            variants.append(vm.group(1))
    # 构造参数：变体名 -> (arg1, arg2, ...)
    args_map = {}
    for line in inner.splitlines():
        line = line.strip()
        vm = re.match(r"(\w+)\s*\(([^)]*)\)", line)
        if vm:
            args = [a.strip().strip('"') for a in vm.group(2).split(",") if a.strip()]
            args_map[vm.group(1)] = tuple(args)
    # 字段：private final Type name;
    fields = []
    for fm in re.finditer(r"private final\s+(\w+)\s+(\w+);", src):
        fields.append((fm.group(2), fm.group(1)))  # (name, type)

    lines = []
    lines.append(f"//! 对应 Java `com.github.binarywang.wxpay.bean.{pkg}.enums.{name}`。")
    lines.append("//!")
    lines.append("//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名")
    lines.append("//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），")
    lines.append("//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。")
    lines.append("")
    lines.append("#![allow(non_camel_case_types)]")
    lines.append("")
    lines.append(f"/// {name}（对应 Java `{name}`）。")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]")
    lines.append(f"pub enum {name} {{")
    for v in variants:
        lines.append(f"    /// {v}")
        lines.append(f"    {v},")
    lines.append("}")
    if fields:
        lines.append("")
        lines.append(f"impl {name} {{")
        for fname, ftype in fields:
            getter = fname
            lines.append(f"    /// 获取 {fname}（对应 Java `get{fname[:1].upper()}{fname[1:]}()`，Lombok @Getter）。")
            lines.append(f"    pub fn {getter}(&self) -> &'static str {{")
            lines.append("        match self {")
            for v in variants:
                arg = args_map.get(v, ("",))[fields.index((fname, ftype))]
                lines.append(f"            {name}::{v} => \"{arg}\",")
            lines.append("        }")
            lines.append("    }")
        lines.append("}")
    return "\n".join(lines)


def main():
    for pkg in PACKAGES:
        d = os.path.join(BEAN, pkg, "enums")
        if not os.path.isdir(d):
            continue
        outdir = os.path.join(OUT, pkg, "enums")
        os.makedirs(outdir, exist_ok=True)
        mod_lines = [f"//! 对应 Java `com.github.binarywang.wxpay.bean.{pkg}.enums` 包（生成）。", ""]
        for f in sorted(os.listdir(d)):
            if not f.endswith(".java"):
                continue
            name = f[:-5]
            out = os.path.join(outdir, snake(name) + ".rs")
            open(out, "w", encoding="utf-8").write(gen_one(os.path.join(d, f), pkg))
            mod_lines.append(f"pub mod {snake(name)};")
            mod_lines.append(f"pub use {snake(name)}::{name};")
            print("wrote", out)
        open(os.path.join(outdir, "mod.rs"), "w", encoding="utf-8").write("\n".join(mod_lines) + "\n")
        pmod = os.path.join(OUT, pkg, "mod.rs")
        s = open(pmod, encoding="utf-8").read()
        if "pub mod enums;" not in s:
            idx = s.index("pub mod ")
            s = s[:idx] + "pub mod enums;\n" + s[idx:]
            open(pmod, "w", encoding="utf-8").write(s)
            print("registered enums in", pmod)


if __name__ == "__main__":
    main()
