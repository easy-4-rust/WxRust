#!/usr/bin/env python3
"""Build a complete declared-method inventory from compiled WxJava Oracle classes.

The denominator is public/protected methods declared by classes/interfaces in the
nine selected Maven modules. Constructors are emitted to a separate CSV. Native
methods are included. JVM descriptors provide stable overload identity.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import os
import re
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from collections import Counter, defaultdict
from pathlib import Path

ORACLE_COMMIT = "a49d6e1461752c06b752d2afd8aeeb7e6e78cefe"
MODULES = (
    "weixin-java-common", "weixin-java-mp", "weixin-java-miniapp",
    "weixin-java-pay", "weixin-java-cp", "weixin-java-open",
    "weixin-java-channel", "weixin-java-aispeech", "weixin-java-qidian",
)
MODIFIERS = {
    "public", "protected", "abstract", "static", "final", "synchronized",
    "native", "strictfp", "default", "transient", "volatile",
}
DECL_RE = re.compile(
    r"^\s*(?P<prefix>.+?)\s+(?P<name>[A-Za-z_$][\w$]*)\s*"
    r"\((?P<params>.*)\)(?:\s+throws\s+(?P<throws>.*?))?;\s*$"
)
DESCRIPTOR_RE = re.compile(r"^\s*descriptor:\s*(\S+)\s*$")
CLASS_RE = re.compile(r"^(?:public|protected|abstract|final|strictfp|sealed|non-sealed|static|\s)*"
                      r"(?:class|interface|enum|@interface)\s+([^\s<{]+)(.*)$")


def sha_id(*parts: str) -> str:
    return hashlib.sha256("\x1f".join(parts).encode("utf-8")).hexdigest()


def class_names(module_dir: Path) -> list[str]:
    classes_dir = module_dir / "target" / "classes"
    if not classes_dir.is_dir():
        raise RuntimeError(f"compiled classes not found: {classes_dir}; build the pinned Maven modules first")
    names: list[str] = []
    for path in classes_dir.rglob("*.class"):
        rel = path.relative_to(classes_dir).as_posix()
        if rel.endswith("package-info.class") or rel.endswith("module-info.class"):
            continue
        stem = path.stem
        # Keep named nested classes; exclude anonymous/local compiler artifacts.
        if re.search(r"\$\d", stem):
            continue
        names.append(rel[:-6].replace("/", "."))
    return sorted(set(names))


def javap(class_name: str, classpath: str, javap_bin: str, timeout: int) -> list[str]:
    proc = subprocess.run(
        [javap_bin, "-classpath", classpath, "-protected", "-s", class_name],
        capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=timeout,
    )
    if proc.returncode != 0:
        raise RuntimeError(f"javap failed for {class_name}: {proc.stderr.strip()}")
    return proc.stdout.splitlines()


def split_prefix(prefix: str) -> tuple[str, str, str]:
    tokens = prefix.strip().split()
    modifiers: list[str] = []
    while tokens and tokens[0] in MODIFIERS:
        modifiers.append(tokens.pop(0))
    rest = " ".join(tokens).strip()
    generic = ""
    if rest.startswith("<"):
        depth = 0
        end = -1
        for i, ch in enumerate(rest):
            if ch == "<":
                depth += 1
            elif ch == ">":
                depth -= 1
                if depth == 0:
                    end = i
                    break
        if end >= 0:
            generic = rest[:end + 1]
            rest = rest[end + 1:].strip()
    return " ".join(modifiers), rest, generic


def parse_class_header(lines: list[str], class_name: str) -> str:
    for line in lines:
        stripped = line.strip()
        if re.match(r"^(?:public|protected)\s+(?:(?:abstract|final|static|strictfp|sealed|non-sealed)\s+)*(?:class|interface|enum|@interface)\s+", stripped):
            return stripped
    return class_name


def parse_methods(lines: list[str], module: str, class_name: str,
                  oracle_commit: str) -> tuple[list[dict[str, str]], list[dict[str, str]], str]:
    methods: list[dict[str, str]] = []
    constructors: list[dict[str, str]] = []
    class_header = parse_class_header(lines, class_name)
    class_is_public_api = class_header.startswith(("public ", "protected "))
    if not class_is_public_api:
        return methods, constructors, class_header

    pending: dict[str, str] | None = None
    simple_name = class_name.rsplit(".", 1)[-1].rsplit("$", 1)[-1]
    constructor_re = re.compile(
        r"^\s*(?P<modifiers>(?:(?:public|protected|static|final|synchronized)\s+)+)"
        r"(?P<qualified>[A-Za-z_$][\w.$]*)\s*\((?P<params>.*)\)"
        r"(?:\s+throws\s+(?P<throws>.*?))?;\s*$"
    )

    for line in lines:
        desc_match = DESCRIPTOR_RE.match(line)
        if desc_match and pending is not None:
            descriptor = desc_match.group(1)
            name = pending["method"]
            row = {
                "method_id": sha_id(module, class_name, name, descriptor),
                "module": module,
                "class": class_name,
                "declared_in": class_name,
                "method": name,
                "modifiers": pending["modifiers"],
                "return_type": pending["return_type"],
                "params": pending["params"],
                "throws": pending["throws"],
                "descriptor": descriptor,
                "generic_signature": pending["generic_signature"],
                "overload_index": "",
                "method_kind": pending["method_kind"],
                "class_header": class_header,
                "oracle_commit": oracle_commit,
            }
            if pending["method_kind"] == "CONSTRUCTOR":
                constructors.append(row)
            else:
                methods.append(row)
            pending = None
            continue

        if not line.strip().endswith(";") or "(" not in line or ")" not in line:
            continue

        ctor = constructor_re.match(line)
        if ctor:
            qualified = ctor.group("qualified")
            ctor_name = qualified.rsplit(".", 1)[-1].rsplit("$", 1)[-1]
            if ctor_name == simple_name:
                modifiers = ctor.group("modifiers").strip()
                pending = {
                    "method": simple_name,
                    "modifiers": modifiers,
                    "return_type": "<constructor>",
                    "params": (ctor.group("params") or "").strip(),
                    "throws": (ctor.group("throws") or "").strip(),
                    "generic_signature": "",
                    "method_kind": "CONSTRUCTOR",
                }
                continue

        match = DECL_RE.match(line)
        if not match:
            continue
        prefix = match.group("prefix").strip()
        name = match.group("name")
        modifiers, return_type, generic = split_prefix(prefix)
        if not ({"public", "protected"} & set(modifiers.split())):
            continue
        method_kind = "NATIVE" if "native" in modifiers.split() else (
            "STATIC" if "static" in modifiers.split() else (
                "ABSTRACT" if "abstract" in modifiers.split() else "INSTANCE"
            )
        )
        pending = {
            "method": name,
            "modifiers": modifiers,
            "return_type": return_type,
            "params": (match.group("params") or "").strip(),
            "throws": (match.group("throws") or "").strip(),
            "generic_signature": generic,
            "method_kind": method_kind,
        }

    return methods, constructors, class_header


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--java-root", required=True)
    ap.add_argument("--output", required=True)
    ap.add_argument("--constructors-output", required=True)
    ap.add_argument("--modules", nargs="*", choices=MODULES, default=list(MODULES))
    ap.add_argument("--oracle-commit", default=ORACLE_COMMIT)
    ap.add_argument("--javap", default="javap")
    ap.add_argument("--timeout", type=int, default=60)
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args()

    all_methods: list[dict[str, str]] = []
    all_constructors: list[dict[str, str]] = []
    counts: Counter[str] = Counter()
    class_counts: Counter[str] = Counter()
    inheritance_rows: list[dict[str, str]] = []

    def inspect_one(module: str, cp: str, class_name: str):
        lines = javap(class_name, cp, args.javap, args.timeout)
        methods, constructors, class_header = parse_methods(
            lines, module, class_name, args.oracle_commit
        )
        return class_name, methods, constructors, class_header

    for module in args.modules:
        module_dir = Path(args.java_root) / module
        cp = str(module_dir / "target" / "classes")
        classes = class_names(module_dir)
        with ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
            futures = [pool.submit(inspect_one, module, cp, class_name) for class_name in classes]
            for future in as_completed(futures):
                class_name, methods, constructors, class_header = future.result()
                all_methods.extend(methods)
                all_constructors.extend(constructors)
                counts[module] += len(methods)
                class_counts[module] += 1
                inheritance_rows.append({
                    "module": module, "class": class_name,
                    "class_header": class_header, "oracle_commit": args.oracle_commit,
                })

    all_methods.sort(key=lambda row: (row["module"], row["class"], row["method"], row["descriptor"]))
    all_constructors.sort(key=lambda row: (row["module"], row["class"], row["method"], row["descriptor"]))
    inheritance_rows.sort(key=lambda row: (row["module"], row["class"]))

    # Stable overload ordinal within each declaration class and method name.
    grouped: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for row in all_methods:
        grouped[(row["class"], row["method"])].append(row)
    for rows in grouped.values():
        for index, row in enumerate(sorted(rows, key=lambda x: x["descriptor"]), start=1):
            row["overload_index"] = str(index)

    method_fields = [
        "method_id", "module", "class", "declared_in", "method", "modifiers",
        "return_type", "params", "throws", "descriptor", "generic_signature",
        "overload_index", "method_kind", "class_header", "oracle_commit",
    ]
    constructor_fields = method_fields
    for output, fields, rows in (
        (Path(args.output), method_fields, all_methods),
        (Path(args.constructors_output), constructor_fields, all_constructors),
    ):
        output.parent.mkdir(parents=True, exist_ok=True)
        with output.open("w", encoding="utf-8", newline="") as f:
            writer = csv.DictWriter(f, fieldnames=fields)
            writer.writeheader()
            writer.writerows(rows)

    inheritance_path = Path(args.output).with_name("inventory_java_class_hierarchy.csv")
    with inheritance_path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["module", "class", "class_header", "oracle_commit"])
        writer.writeheader()
        writer.writerows(inheritance_rows)

    print("=== WxJava declared method inventory ===")
    for module in args.modules:
        print(f"{module:24s} classes={class_counts[module]:5d} methods={counts[module]:7d}")
    print(f"TOTAL_CLASSES {sum(class_counts.values())}")
    print(f"TOTAL_METHODS {len(all_methods)}")
    print(f"TOTAL_CONSTRUCTORS {len(all_constructors)}")
    print(f"METHODS_CSV {args.output}")
    print(f"CONSTRUCTORS_CSV {args.constructors_output}")
    print(f"HIERARCHY_CSV {inheritance_path}")

    ids = [row["method_id"] for row in all_methods]
    if len(ids) != len(set(ids)):
        raise SystemExit("[fatal] duplicate method_id values")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
