#!/usr/bin/env python3
"""B0 方法/签名/参数分母清点（v2）：javap 枚举 WxJava 各模块 class 的公共方法签名。

v2 修复：
- javap 加 -classpath（模块 target/classes）
- 方法行解析适配 javap -public -s 实际格式（含泛型/数组/throws/descriptor 行）
- 输出统一用 JDK 8 javap

用法: inventory_java_methods.py --java-root <WxJava> --output <csv> [--modules m1 m2...]
"""
import argparse
import csv
import os
import re
import subprocess
import sys
from collections import Counter

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


def find_classes(module_dir, pkg_root):
    base = os.path.join(module_dir, "target", "classes", pkg_root)
    if not os.path.isdir(base):
        return []
    out = []
    for dirpath, _, files in os.walk(base):
        for fn in files:
            if fn.endswith(".class") and "$" not in fn and not fn.startswith("package-info"):
                full = os.path.join(dirpath, fn)
                rel = os.path.relpath(full, os.path.join(module_dir, "target", "classes"))
                out.append(rel[:-6].replace(os.sep, "."))
    return sorted(out)


def javap_signatures(cls_name, classpath):
    try:
        r = subprocess.run(["javap", "-classpath", classpath, "-public", "-s", cls_name],
                           capture_output=True, text=True, timeout=60)
        if r.returncode != 0:
            return []
        return r.stdout.splitlines()
    except Exception:
        return []


# javap -public 方法行（非 descriptor 行）：public/protected [static/abstract/final/synchronized/native] [generic] return name(params) [throws ...];
# 注意返回类型可能含泛型 <T>、[]；参数可能含泛型与 ...（varargs）
METHOD_RE = re.compile(
    r"^\s*(public|protected)\s+"
    r"(?:(?:abstract|static|final|synchronized|native)\s+)*"   # 修饰符（任意顺序）
    r"(?:<[\w,\s.\[\]?]+>\s+)?"                                 # 泛型方法（可选）
    r"([\w.<>\[\],\s?]+)\s+"                                    # 返回类型（保守捕获）
    r"([A-Za-z_$][\w$]*)\s*\("                                  # 方法名
    r"([^)]*)\)"                                                # 参数
    r"(?:\s+throws\s+([\w.,\s]+))?\s*;"                        # throws（可选）
)


def parse_methods(lines):
    """javap 输出 -> [(name, params, throws)]，跳过构造器。"""
    methods = []
    for line in lines:
        line = line.rstrip()
        if line.startswith(("  ", "\t")) and "(" in line and ")" in line:
            m = METHOD_RE.match(line)
            if not m:
                continue
            _, _ret, name, params, throws = m.groups()
            if name in ("<init>", "<clinit>"):
                continue
            methods.append((name, params.strip(), (throws or "").strip()))
    return methods


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--java-root", required=True)
    ap.add_argument("--output", required=True)
    ap.add_argument("--modules", nargs="*", default=None)
    args = ap.parse_args()

    rows = []
    summary = Counter()
    for module, pkg_root in MODULE_PACKAGE_ROOTS.items():
        if args.modules and module not in args.modules:
            continue
        module_dir = os.path.join(args.java_root, module)
        cp = os.path.join(module_dir, "target", "classes")
        classes = find_classes(module_dir, pkg_root)
        n_methods = 0
        for cls in classes:
            lines = javap_signatures(cls, cp)
            for name, params, throws in parse_methods(lines):
                rows.append({"module": module, "class": cls, "method": name,
                             "params": params, "throws": throws})
                n_methods += 1
        summary[module] = (len(classes), n_methods)
        print(f"{module:24s} classes={len(classes):4d} methods={n_methods:5d}")

    with open(args.output, "w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=["module", "class", "method", "params", "throws"])
        w.writeheader()
        w.writerows(rows)
    print(f"\nCSV -> {args.output}  total methods: {len(rows)}")


if __name__ == "__main__":
    main()
