#!/usr/bin/env python3
"""WxJava Java 对象清点脚本：枚举全部 main Java 对象 + 期望 Rust 路径。

路径算法（rust-java-migration 技能）：
- 从 Java 包根起计算相对包路径，保留末 retain_segments(2) 层
- 文件名转 snake_case；类型名 PascalCase 转 snake_case
- 内部类默认不单独成文件（随主对象）；独立 enum/interface/class 各自一行

输出：CSV（--csv 指定路径）或 Markdown 表（--markdown）
"""
import argparse
import csv
import os
import re
from collections import Counter, defaultdict

# 各模块的包根（WxJava 有 5 个不同的组织根）
MODULE_PACKAGE_ROOTS = {
    "weixin-java-common":   "me/chanjar/weixin/common",
    "weixin-java-mp":       "me/chanjar/weixin/mp",
    "weixin-java-miniapp":  "cn/binarywang/wx/miniapp",
    "weixin-java-pay":      "com/github/binarywang/wxpay",
    "weixin-java-cp":       "me/chanjar/weixin/cp",
    "weixin-java-open":     "me/chanjar/weixin/open",
    "weixin-java-channel":  "me/chanjar/weixin/channel",
    "weixin-java-aispeech": "me/chanjar/weixin/aispeech",
    "weixin-java-qidian":   "me/chanjar/weixin/qidian",
}

# 支持 public / 包私有（无修饰符）顶层类型；排除 package-info
TYPE_PATTERNS = [
    ("class",       re.compile(r"^\s*(?:public\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+([A-Z]\w*)")),
    ("interface",   re.compile(r"^\s*(?:public\s+)?interface\s+([A-Z]\w*)")),
    ("enum",        re.compile(r"^\s*(?:public\s+)?enum\s+([A-Z]\w*)")),
    ("annotation",  re.compile(r"^\s*(?:public\s+)?@interface\s+([A-Z]\w*)")),
    ("record",      re.compile(r"^\s*(?:public\s+)?record\s+([A-Z]\w*)")),
]


def snake(name: str) -> str:
    """PascalCase -> snake_case"""
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def camel_to_snake(name: str) -> str:
    return snake(name)


def find_type(filepath: str):
    """返回 (type_kind, type_name) 或 None。取文件中第一个 public 顶层类型。"""
    with open(filepath, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("//") or line.startswith("/*") or line.startswith("*"):
                continue
            for kind, pat in TYPE_PATTERNS:
                m = pat.match(line)
                if m:
                    return kind, m.group(1)
            # 泛型 class 带继承、带泛型参数（含包私有）
            m = re.match(r"^\s*(?:public\s+)?(?:abstract\s+|final\s+|static\s+)*class\s+([A-Z]\w*)\s*[<{]", line)
            if m:
                return "class", m.group(1)
    return None


def expected_rust_path(rel_dir: str, type_name: str, retain: int) -> str:
    """从包根起的相对目录 + 文件名 -> Rust 路径（保留末 retain 层）"""
    parts = [p for p in rel_dir.split("/") if p]
    if len(parts) > retain:
        parts = parts[-retain:]
    file = camel_to_snake(type_name)
    path_parts = parts + [file + ".rs"]
    return "/".join(path_parts)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--java-root", required=True, help="WxJava 仓库根")
    ap.add_argument("--retain-segments", type=int, default=2)
    ap.add_argument("--csv", help="输出 CSV 路径")
    ap.add_argument("--markdown", help="输出 Markdown 路径")
    ap.add_argument("--include-tests", action="store_true", help="同时清点 test 目录")
    args = ap.parse_args()

    rows = []
    summary = defaultdict(Counter)

    for module, pkg_root in MODULE_PACKAGE_ROOTS.items():
        main_dir = os.path.join(args.java_root, module, "src/main/java")
        pkg_abs = os.path.join(main_dir, pkg_root)
        if not os.path.isdir(pkg_abs):
            print(f"[skip] {module}: 包根不存在 {pkg_abs}")
            continue

        for dirpath, dirnames, filenames in os.walk(pkg_abs):
            dirnames.sort()
            for fn in sorted(filenames):
                if not fn.endswith(".java"):
                    continue
                full = os.path.join(dirpath, fn)
                found = find_type(full)
                if not found:
                    summary[module]["unparsed"] += 1
                    rows.append({
                        "module": module, "java_file": os.path.relpath(full, main_dir),
                        "kind": "?", "java_name": "?", "rust_path": "?",
                    })
                    continue
                kind, tname = found
                rel_dir = os.path.relpath(dirpath, pkg_abs)
                rust_path = expected_rust_path(rel_dir, tname, args.retain_segments)
                summary[module][kind] += 1
                rows.append({
                    "module": module,
                    "java_file": os.path.relpath(full, main_dir),
                    "kind": kind,
                    "java_name": tname,
                    "rust_path": rust_path,
                })

        if args.include_tests:
            test_dir = os.path.join(args.java_root, module, "src/test/java")
            test_pkg = os.path.join(test_dir, pkg_root)
            if os.path.isdir(test_pkg):
                for dirpath, _, filenames in os.walk(test_pkg):
                    for fn in sorted(filenames):
                        if not fn.endswith(".java") or "package-info" in fn:
                            continue
                        full = os.path.join(dirpath, fn)
                        found = find_type(full)
                        if found:
                            summary[module]["test_" + found[0]] += 1
                        else:
                            summary[module]["test_unparsed"] += 1

    if args.csv:
        with open(args.csv, "w", newline="", encoding="utf-8") as f:
            w = csv.DictWriter(f, fieldnames=["module", "java_file", "kind", "java_name", "rust_path"])
            w.writeheader()
            w.writerows(rows)
        print(f"CSV -> {args.csv}")

    if args.markdown:
        with open(args.markdown, "w", encoding="utf-8") as f:
            f.write("| 模块 | Java 文件 | 类型 | Java 对象 | 预期 Rust 路径 |\n|---|---|---|---|---|\n")
            for r in rows:
                f.write(f"| {r['module']} | `{r['java_file']}` | {r['kind']} | `{r['java_name']}` | `{r['rust_path']}` |\n")
        print(f"Markdown -> {args.markdown}")

    print("\n=== 汇总 ===")
    for m in MODULE_PACKAGE_ROOTS:
        if m in summary:
            c = summary[m]
            total = sum(v for k, v in c.items() if not k.startswith("test"))
            print(f"{m:24s} 对象={total:5d}  " +
                  "  ".join(f"{k}={v}" for k, v in sorted(c.items()) if not k.startswith("test")))
    grand = sum(sum(v for k, v in c.items() if not k.startswith("test")) for c in summary.values())
    print(f"\nmain 对象合计: {grand}")


if __name__ == "__main__":
    main()
