#!/usr/bin/env python3
"""Machine-check Phase 1 inventory and compatibility-matrix closure."""
from __future__ import annotations

import argparse
import csv
import json
from collections import Counter, defaultdict
from pathlib import Path

MODULES = (
    "weixin-java-common", "weixin-java-mp", "weixin-java-miniapp",
    "weixin-java-pay", "weixin-java-cp", "weixin-java-open",
    "weixin-java-channel", "weixin-java-aispeech", "weixin-java-qidian",
)
EXPECTED_COUNTS = {
    "weixin-java-common": 174,
    "weixin-java-mp": 428,
    "weixin-java-miniapp": 611,
    "weixin-java-pay": 570,
    "weixin-java-cp": 595,
    "weixin-java-open": 240,
    "weixin-java-channel": 618,
    "weixin-java-aispeech": 25,
    "weixin-java-qidian": 27,
}
ORACLE_COMMIT = "a49d6e1461752c06b752d2afd8aeeb7e6e78cefe"


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--java-root", required=True)
    ap.add_argument("--objects", required=True)
    ap.add_argument("--methods", required=True)
    ap.add_argument("--matrix", required=True)
    ap.add_argument("--oracle-commit", default=ORACLE_COMMIT)
    ap.add_argument("--report-json")
    args = ap.parse_args()

    java_root = Path(args.java_root)
    objects = read_csv(Path(args.objects))
    methods = read_csv(Path(args.methods))
    matrix = read_csv(Path(args.matrix))
    errors: list[str] = []
    actual_counts: Counter[str] = Counter()

    for module in MODULES:
        source_dir = java_root / module / "src" / "main" / "java"
        actual_counts[module] = sum(1 for _ in source_dir.rglob("*.java")) if source_dir.is_dir() else -1
        if actual_counts[module] != EXPECTED_COUNTS[module]:
            errors.append(
                f"source count mismatch {module}: expected={EXPECTED_COUNTS[module]} actual={actual_counts[module]}"
            )

    object_counts = Counter(row["module"] for row in objects)
    for module in MODULES:
        if object_counts[module] != actual_counts[module]:
            errors.append(
                f"object inventory mismatch {module}: source={actual_counts[module]} inventory={object_counts[module]}"
            )
    if len(objects) != sum(actual_counts.values()):
        errors.append(f"object denominator mismatch: inventory={len(objects)} source={sum(actual_counts.values())}")

    object_keys = [(row["module"], row["java_file"]) for row in objects]
    if len(object_keys) != len(set(object_keys)):
        errors.append("duplicate object inventory keys (module, java_file)")

    finance = [
        row for row in objects
        if row.get("java_fqn") == "com.tencent.wework.Finance"
        or row.get("java_file", "").replace("\\", "/").endswith("com/tencent/wework/Finance.java")
    ]
    if len(finance) != 1:
        errors.append(f"Finance.java must appear exactly once, found={len(finance)}")
    elif finance[0].get("source_scope") != "OUTSIDE_PACKAGE_ROOT":
        errors.append("Finance.java must be marked OUTSIDE_PACKAGE_ROOT")

    method_ids = [row.get("method_id", "") for row in methods]
    if not method_ids or any(not value for value in method_ids):
        errors.append("method inventory has empty method_id or no rows")
    if len(method_ids) != len(set(method_ids)):
        errors.append("duplicate method_id in method inventory")

    matrix_ids = [row.get("method_id", "") for row in matrix]
    if len(matrix) != len(methods):
        errors.append(f"matrix denominator mismatch: methods={len(methods)} matrix={len(matrix)}")
    if len(matrix_ids) != len(set(matrix_ids)):
        errors.append("duplicate method_id in compatibility matrix")
    if set(matrix_ids) != set(method_ids):
        errors.append("method_id set differs between inventory and compatibility matrix")

    statuses = Counter(row.get("migration_status", "") for row in matrix)
    if sum(statuses.values()) != len(matrix):
        errors.append("migration status totals do not sum to matrix denominator")
    if "VERIFIED" in statuses:
        errors.append("automatic matrix contains VERIFIED; this is forbidden")

    finance_methods = [row for row in matrix if row.get("java_class") == "com.tencent.wework.Finance"]
    finance_statuses = Counter(row.get("migration_status", "") for row in finance_methods)
    if not finance_methods:
        errors.append("Finance Native methods absent from compatibility matrix")
    for row in finance_methods:
        if row["java_method"] in {
            "NewSdk", "Init", "GetChatData", "GetMediaData", "DestroySdk",
            "NewSlice", "FreeSlice", "GetContentFromSlice", "GetSliceLen",
            "NewMediaData", "FreeMediaData", "GetOutIndexBuf", "GetData",
            "GetIndexLen", "GetDataLen", "IsMediaDataFinish",
        } and row["migration_status"] != "UNSUPPORTED":
            errors.append(f"Finance.{row['java_method']} must be UNSUPPORTED")
        if row["java_method"] == "DecryptData" and row["migration_status"] != "ADAPTED":
            errors.append("Finance.DecryptData must be ADAPTED pending differential verification")

    report = {
        "oracle_commit": args.oracle_commit,
        "object_source_counts": dict(actual_counts),
        "object_inventory_counts": dict(object_counts),
        "object_total": len(objects),
        "method_total": len(methods),
        "matrix_total": len(matrix),
        "method_status_counts": dict(statuses),
        "finance_method_total": len(finance_methods),
        "finance_status_counts": dict(finance_statuses),
        "errors": errors,
        "passed": not errors,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2))
    if args.report_json:
        output = Path(args.report_json)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
