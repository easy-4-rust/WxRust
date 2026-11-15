#!/usr/bin/env python3
"""Create a conservative Java -> Rust method compatibility candidate matrix.

A filename/type/method-name hit is only a candidate. This script never emits
VERIFIED; semantic parity requires separately attached differential evidence.
"""
from __future__ import annotations

import argparse
import csv
import re
from collections import Counter, defaultdict
from pathlib import Path

MODULE_TO_CRATE = {
    "weixin-java-common": "wx-rust-common",
    "weixin-java-mp": "wx-rust-mp",
    "weixin-java-miniapp": "wx-rust-miniapp",
    "weixin-java-pay": "wx-rust-pay",
    "weixin-java-cp": "wx-rust-cp",
    "weixin-java-open": "wx-rust-open",
    "weixin-java-channel": "wx-rust-channel",
    "weixin-java-aispeech": "wx-rust-aispeech",
    "weixin-java-qidian": "wx-rust-qidian",
}
UNSUPPORTED_FINANCE = {
    "NewSdk", "Init", "GetChatData", "GetMediaData", "DestroySdk",
    "NewSlice", "FreeSlice", "GetContentFromSlice", "GetSliceLen",
    "NewMediaData", "FreeMediaData", "GetOutIndexBuf", "GetData",
    "GetIndexLen", "GetDataLen", "IsMediaDataFinish",
}
PLATFORM_FINANCE = {"isWindows", "loadingLibraries", "SingletonSDK", "DestroySingletonSDK"}


def snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def rust_index(root: Path) -> dict[str, dict[str, str]]:
    result: dict[str, dict[str, str]] = {}
    for module, crate in MODULE_TO_CRATE.items():
        src = root / "crates" / crate / "src"
        by_name: dict[str, list[Path]] = defaultdict(list)
        if src.is_dir():
            for path in src.rglob("*.rs"):
                if path.name != "mod.rs":
                    by_name[path.name].append(path)
        for name, paths in by_name.items():
            for path in paths:
                try:
                    text = path.read_text(encoding="utf-8")
                except (OSError, UnicodeDecodeError):
                    continue
                result[f"{module}:{path.relative_to(src).as_posix()}"] = {
                    "module": module,
                    "crate": crate,
                    "path": path.relative_to(root).as_posix(),
                    "basename": name,
                    "text": text,
                }
    return result


def rust_type_names(text: str) -> list[str]:
    pattern = re.compile(r"\bpub\s+(?:\([^)]*\)\s+)?(?:trait|struct|enum|type)\s+([A-Za-z_][A-Za-z0-9_]*)")
    return list(dict.fromkeys(pattern.findall(text)))


def find_candidate(module: str, java_class: str, java_method: str,
                   object_row: dict[str, str] | None,
                   index: dict[str, dict[str, str]]) -> tuple[str, str, str, str]:
    crate = MODULE_TO_CRATE[module]
    if not object_row:
        return crate, "", "", "NO_OBJECT_ROW"

    expected = object_row.get("rust_path", "")
    if expected.startswith("@special/") or expected in ("", "?"):
        return crate, "", "", "NO_EXPECTED_RUST_PATH"

    basename = Path(expected).name
    candidates = [
        entry for key, entry in index.items()
        if entry["module"] == module and entry["basename"] == basename
    ]
    if not candidates:
        # Compound-word spelling and moved-file fallback: still only a candidate.
        java_top = java_class.split("$", 1)[0].rsplit(".", 1)[-1]
        expected_names = {snake(java_top) + ".rs"}
        expected_names.update({java_top.lower() + ".rs"})
        candidates = [
            entry for entry in index.values()
            if entry["module"] == module and entry["basename"] in expected_names
        ]
    if not candidates:
        return crate, "", "", "NO_FILE_CANDIDATE"

    # Multiple same-name files are ambiguous; do not select one arbitrarily.
    if len(candidates) != 1:
        return crate, "", "", "AMBIGUOUS_FILE_CANDIDATE"

    entry = candidates[0]
    fn = snake(java_method)
    text = entry["text"]
    fn_pattern = re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+" + re.escape(fn) + r"\s*(?:<|\()")
    type_names = rust_type_names(text)
    if fn_pattern.search(text):
        return crate, "|".join(type_names), entry["path"], "CANDIDATE_MAPPED"
    return crate, "|".join(type_names), entry["path"], "CANDIDATE_FILE_ONLY"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--project-root", default=".")
    ap.add_argument("--objects", required=True)
    ap.add_argument("--methods", required=True)
    ap.add_argument("--output", required=True)
    args = ap.parse_args()

    root = Path(args.project_root)
    objects = read_csv(Path(args.objects))
    methods = read_csv(Path(args.methods))
    object_by_fqn = {
        (row.get("module", ""), row.get("java_fqn", "")): row for row in objects
    }
    # Legacy rows may lack FQN; use the package path + primary type as fallback.
    for row in objects:
        fqn = row.get("java_fqn") or ""
        if not fqn and row.get("java_name") not in ("", "?"):
            fqn = row.get("java_name", "")
        object_by_fqn.setdefault((row.get("module", ""), fqn), row)

    index = rust_index(root)
    output_rows: list[dict[str, str]] = []
    statuses: Counter[str] = Counter()

    for method in methods:
        module = method["module"]
        java_class = method["class"]
        java_name = method["method"]
        top_class = java_class.split("$", 1)[0]
        obj = object_by_fqn.get((module, top_class))
        if obj is None:
            obj = object_by_fqn.get((module, top_class.rsplit(".", 1)[-1]))

        migration_status = "NEEDS_REVIEW"
        semantic_status = "NOT_VERIFIED"
        difference = "自动候选仅依据名称/路径，未证明行为等价。"
        test_reference = ""
        rust_crate, rust_type, rust_path, candidate_status = find_candidate(
            module, java_class, java_name, obj, index
        )

        if java_class == "com.tencent.wework.Finance":
            if java_name in UNSUPPORTED_FINANCE:
                migration_status = "UNSUPPORTED"
                semantic_status = "UNSUPPORTED"
                difference = "WxRust 未实现官方 Finance Native SDK 句柄/私有协议能力；不得计入兼容完成。"
            elif java_name == "DecryptData":
                migration_status = "ADAPTED"
                semantic_status = "ADAPTED_PENDING"
                rust_crate, rust_type, rust_path = (
                    "wx-rust-cp", "WxCpMsgAuditServiceImpl",
                    "crates/wx-rust-cp/src/api/impl/wx_cp_msg_audit_service_impl.rs",
                )
                difference = "Rust 以纯 RSA+AES 解密服务替代 Finance.DecryptData；需逐样本差分验证。"
            elif java_name in PLATFORM_FINANCE:
                migration_status = "PLATFORM_NA"
                semantic_status = "PLATFORM_NA"
                difference = "Java DLL/SO 加载和 Native SDK 单例/句柄生命周期机制；Rust 当前不提供同一 Native ABI。"
        elif candidate_status == "CANDIDATE_MAPPED":
            migration_status = "CANDIDATE_MAPPED"
            difference = "候选 Rust 文件存在同名 snake_case 方法；仍需人工核对签名、返回值和行为。"
        elif candidate_status == "CANDIDATE_FILE_ONLY":
            migration_status = "CANDIDATE_FILE_ONLY"
            difference = "仅定位到候选 Rust 文件/类型，未找到同名方法；可能是字段、组合实现或缺口。"
        elif candidate_status == "AMBIGUOUS_FILE_CANDIDATE":
            migration_status = "NEEDS_REVIEW"
            difference = "同名 Rust 文件存在多个候选，需人工选择映射。"
        else:
            difference = f"未找到可靠 Rust 候选：{candidate_status}。"

        row = {
            "method_id": method.get("method_id", ""),
            "module": module,
            "java_class": java_class,
            "declared_in": method.get("declared_in", java_class),
            "java_class_header": method.get("class_header", ""),
            "java_method": java_name,
            "java_modifiers": method.get("modifiers", ""),
            "java_return_type": method.get("return_type", ""),
            "java_params": method.get("params", ""),
            "java_throws": method.get("throws", ""),
            "java_descriptor": method.get("descriptor", ""),
            "java_generic_signature": method.get("generic_signature", ""),
            "overload_index": method.get("overload_index", ""),
            "method_kind": method.get("method_kind", ""),
            "rust_crate": rust_crate,
            "rust_type_candidate": rust_type,
            "rust_file_candidate": rust_path,
            "rust_method_candidate": snake(java_name),
            "migration_status": migration_status,
            "semantic_status": semantic_status,
            "test_reference": test_reference,
            "difference": difference,
            "oracle_commit": method.get("oracle_commit", ""),
        }
        output_rows.append(row)
        statuses[migration_status] += 1

    fields = list(output_rows[0].keys()) if output_rows else [
        "method_id", "module", "java_class", "declared_in", "java_class_header", "java_method",
        "java_modifiers", "java_return_type", "java_params", "java_throws",
        "java_descriptor", "java_generic_signature", "overload_index", "method_kind",
        "rust_crate", "rust_type_candidate", "rust_file_candidate",
        "rust_method_candidate", "migration_status", "semantic_status",
        "test_reference", "difference", "oracle_commit",
    ]
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        writer.writerows(output_rows)

    print("=== Method compatibility matrix ===")
    print(f"METHOD_ROWS {len(methods)}")
    print(f"MATRIX_ROWS {len(output_rows)}")
    for status, count in sorted(statuses.items()):
        print(f"{status} {count}")
    print(f"OUTPUT {output}")

    if len(output_rows) != len(methods):
        raise SystemExit("[fatal] matrix row count differs from method inventory")
    ids = [row["method_id"] for row in output_rows]
    if len(ids) != len(set(ids)):
        raise SystemExit("[fatal] duplicate method_id in matrix")
    if any(row["semantic_status"] == "VERIFIED" for row in output_rows):
        raise SystemExit("[fatal] automatic matrix must never mark VERIFIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
