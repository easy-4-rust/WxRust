#!/usr/bin/env python3
"""Phase 1 authoritative source inventory for the pinned WxJava Oracle.

Counts every *.java file under the nine selected modules' src/main/java trees.
Unlike the legacy inventory, it does not constrain discovery to one package root,
so package-root outliers such as com.tencent.wework.Finance are retained.
"""
from __future__ import annotations

import argparse
import csv
import os
import re
from collections import Counter
from pathlib import Path

ORACLE_COMMIT = "a49d6e1461752c06b752d2afd8aeeb7e6e78cefe"
MODULES = {
    "weixin-java-common": ("me/chanjar/weixin/common", "wx-rust-common"),
    "weixin-java-mp": ("me/chanjar/weixin/mp", "wx-rust-mp"),
    "weixin-java-miniapp": ("cn/binarywang/wx/miniapp", "wx-rust-miniapp"),
    "weixin-java-pay": ("com/github/binarywang/wxpay", "wx-rust-pay"),
    "weixin-java-cp": ("me/chanjar/weixin/cp", "wx-rust-cp"),
    "weixin-java-open": ("me/chanjar/weixin/open", "wx-rust-open"),
    "weixin-java-channel": ("me/chanjar/weixin/channel", "wx-rust-channel"),
    "weixin-java-aispeech": ("me/chanjar/weixin/aispeech", "wx-rust-aispeech"),
    "weixin-java-qidian": ("me/chanjar/weixin/qidian", "wx-rust-qidian"),
}
TYPE_RE = re.compile(
    r"^\s*(?:(?:public|protected|private|abstract|final|static|sealed|non-sealed)\s+)*"
    r"(@interface|class|interface|enum|record)\s+([A-Za-z_$][\w$]*)",
    re.MULTILINE,
)
PACKAGE_RE = re.compile(r"^\s*package\s+([A-Za-z_$][\w$.]*)\s*;", re.MULTILINE)
COMMENT_RE = re.compile(r"/\*.*?\*/|^\s*//.*?$", re.DOTALL | re.MULTILINE)


def parse_java_file(path: Path) -> tuple[str, str, str, int]:
    text = path.read_text(encoding="utf-8", errors="replace")
    package_match = PACKAGE_RE.search(text)
    package_name = package_match.group(1) if package_match else ""
    if path.name == "package-info.java":
        return package_name, "package-info", "package-info", 0
    if path.name == "module-info.java":
        return "", "module-info", "module-info", 0

    cleaned = COMMENT_RE.sub("", text)
    declarations = list(TYPE_RE.finditer(cleaned))
    if declarations:
        kind_token, primary_name = declarations[0].groups()
        kind = "annotation" if kind_token == "@interface" else kind_token
        names = [m.group(2) for m in declarations]
        return package_name, kind, primary_name, len(names)

    return package_name, "unparsed", path.stem, 0


def expected_rust_path(java_file: str, package_root: str, package_name: str,
                       primary_name: str, retain_segments: int) -> tuple[str, str]:
    normalized = java_file.replace("\\", "/")
    root = package_root.strip("/")
    if normalized.startswith(root + "/"):
        rel = normalized[len(root) + 1:]
        parts = rel.split("/")[:-1]
        if len(parts) > retain_segments:
            parts = parts[-retain_segments:]
        stem = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", primary_name)
        stem = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", stem).lower()
        return "/".join(parts + [stem + ".rs"]), "PACKAGE_ROOT"
    if primary_name == "Finance" and package_name == "com.tencent.wework":
        return "@special/native-finance-sdk", "OUTSIDE_PACKAGE_ROOT"
    return "@special/" + normalized.replace("/", "_"), "OUTSIDE_PACKAGE_ROOT"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--java-root", required=True)
    ap.add_argument("--csv", required=True)
    ap.add_argument("--retain-segments", type=int, default=2)
    ap.add_argument("--oracle-commit", default=ORACLE_COMMIT)
    args = ap.parse_args()

    java_root = Path(args.java_root)
    rows: list[dict[str, str | int]] = []
    per_module: Counter[str] = Counter()
    for module, (package_root, rust_crate) in MODULES.items():
        main_dir = java_root / module / "src" / "main" / "java"
        if not main_dir.is_dir():
            raise SystemExit(f"[fatal] missing source directory: {main_dir}")
        files = sorted(main_dir.rglob("*.java"), key=lambda p: p.as_posix().lower())
        if not files:
            raise SystemExit(f"[fatal] no Java source files found: {main_dir}")
        for path in files:
            rel = path.relative_to(main_dir).as_posix()
            package_name, kind, java_name, type_count = parse_java_file(path)
            rust_path, source_scope = expected_rust_path(
                rel, package_root, package_name, java_name, args.retain_segments
            )
            fqn = (package_name + "." + java_name) if package_name else java_name
            rows.append({
                "module": module,
                "java_file": rel,
                "kind": kind,
                "java_name": java_name,
                "rust_path": rust_path,
                "java_package": package_name,
                "java_fqn": fqn,
                "type_declaration_count": type_count,
                "rust_crate": rust_crate,
                "source_scope": source_scope,
                "oracle_commit": args.oracle_commit,
            })
            per_module[module] += 1

    fields = [
        "module", "java_file", "kind", "java_name", "rust_path",
        "java_package", "java_fqn", "type_declaration_count", "rust_crate",
        "source_scope", "oracle_commit",
    ]
    output = Path(args.csv)
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)

    print("=== WxJava source inventory ===")
    for module in MODULES:
        print(f"{module:24s} {per_module[module]:5d}")
    total = sum(per_module.values())
    print(f"TOTAL {total}")
    print(f"CSV {output}")
    print(f"Oracle {args.oracle_commit}")
    duplicates = Counter((r["module"], r["java_file"]) for r in rows)
    duplicate_count = sum(1 for n in duplicates.values() if n != 1)
    if duplicate_count:
        raise SystemExit(f"[fatal] duplicate source rows: {duplicate_count}")
    if total != len(rows):
        raise SystemExit(f"[fatal] row count mismatch: total={total}, rows={len(rows)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
