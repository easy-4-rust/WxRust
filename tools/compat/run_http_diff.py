#!/usr/bin/env python3
"""Run the pinned WxJava MiniApp HTTP stack and WxRust MiniApp on one loopback fixture.

Exit codes: 0 = strict trace equivalence, 1 = SDK behavior differs, 2 = infrastructure/provenance error.
All network traffic is confined to 127.0.0.1 inside each adapter process.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
from urllib.parse import urlparse
from urllib.request import url2pathname

COMPAT_DIR = Path(__file__).resolve().parent
if str(COMPAT_DIR) not in sys.path:
    sys.path.insert(0, str(COMPAT_DIR))
from compare import compare_records, load_jsonl  # noqa: E402

ORACLE = "a49d6e1461752c06b752d2afd8aeeb7e6e78cefe"
ROOT = Path(__file__).resolve().parents[2]
DEFAULT_FIXTURE = ROOT / "tools/compat/http/fixtures.jsonl"
OPERATIONS = {
    "miniapp.http.post-replay",
    "miniapp.http.get-query-order",
    "miniapp.http.get-custom-client-headers",
    "miniapp.http.get-dynamic-interceptor",
    "miniapp.http.media-upload",
    "miniapp.http.common-upload",
    "miniapp.http.media-download",
    "miniapp.http.media-download-file",
    "miniapp.http.post-xml-response",
    "miniapp.http.post-empty-response",
}
GET_OPERATIONS = {"miniapp.http.get-query-order", "miniapp.http.get-custom-client-headers", "miniapp.http.get-dynamic-interceptor"}
POST_OPERATIONS = {"miniapp.http.post-replay", "miniapp.http.post-xml-response", "miniapp.http.post-empty-response"}
UPLOAD_OPERATIONS = {"miniapp.http.media-upload", "miniapp.http.common-upload"}
DOWNLOAD_OPERATIONS = {"miniapp.http.media-download", "miniapp.http.media-download-file"}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_digest() -> str:
    paths = [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
    for directory in ("crates/wx-rust-common", "crates/wx-rust-miniapp", "tools/compat"):
        paths += [
            path for path in (ROOT / directory).rglob("*")
            if path.is_file()
            and path.suffix in (".rs", ".toml", ".java", ".py", ".jsonl")
            and "target" not in path.parts
            and "__pycache__" not in path.parts
        ]
    digest = hashlib.sha256()
    for path in sorted(set(paths)):
        if path.exists():
            digest.update(path.relative_to(ROOT).as_posix().encode("utf-8"))
            digest.update(bytes.fromhex(sha(path)))
    return digest.hexdigest()


def response_parts(value, label: str) -> tuple[int, str, str]:
    if isinstance(value, str):
        return 200, value, "application/json; charset=UTF-8"
    if isinstance(value, dict):
        status = value.get("status", 200)
        body = value.get("body")
        content_type = value.get("content_type", "application/json; charset=UTF-8")
        if not isinstance(status, int) or not 100 <= status <= 599:
            raise ValueError(f"{label}: status must be an HTTP status integer")
        if not isinstance(body, str):
            raise ValueError(f"{label}: body must be a string")
        if not isinstance(content_type, str) or not content_type:
            raise ValueError(f"{label}: content_type must be a non-empty string")
        return status, body, content_type
    raise ValueError(f"{label}: response must be a string or {{status, body, content_type}} object")


def load_http_fixtures(path: Path) -> list[dict]:
    rows = load_jsonl(path)
    if not rows:
        raise ValueError("empty HTTP fixture set")
    ids = set()
    required = (
        "app_id", "app_secret", "token_path", "api_path", "api_query",
        "default_headers", "token_responses", "api_responses",
    )
    expected_default_headers = None
    for row in rows:
        if not isinstance(row, dict):
            raise ValueError("fixture row must be an object")
        case_id = row.get("id")
        if not isinstance(case_id, str) or not case_id:
            raise ValueError("fixture requires a non-empty id")
        if case_id in ids:
            raise ValueError(f"duplicate fixture id: {case_id}")
        ids.add(case_id)
        operation = row.get("operation")
        if operation not in OPERATIONS:
            raise ValueError(f"unsupported HTTP operation: {operation}")
        data = row.get("input")
        if not isinstance(data, dict):
            raise ValueError(f"{case_id}: input must be an object")
        for key in required:
            if key not in data:
                raise ValueError(f"{case_id}: missing input field {key}")
        for key in required[:5]:
            if not isinstance(data[key], str) or not data[key]:
                raise ValueError(f"{case_id}: {key} must be a non-empty string")
        if not isinstance(data["default_headers"], dict):
            raise ValueError(f"{case_id}: default_headers must be an object")
        dynamic_headers = data.get("dynamic_headers")
        if dynamic_headers is not None:
            if not isinstance(dynamic_headers, dict) or set(dynamic_headers) != {"name", "token_value", "api_value"}:
                raise ValueError(f"{case_id}: dynamic_headers must contain name, token_value and api_value")
            if any(not isinstance(dynamic_headers[key], str) or not dynamic_headers[key] for key in dynamic_headers):
                raise ValueError(f"{case_id}: dynamic header name and values must be non-empty strings")
            if any(ch in dynamic_headers["name"] for ch in "\r\n:"):
                raise ValueError(f"{case_id}: invalid dynamic header name")
            if any(ch in dynamic_headers[key] for key in ("token_value", "api_value") for ch in "\r\n"):
                raise ValueError(f"{case_id}: invalid dynamic header value")

        forbidden_headers = {"connection", "content-length", "host", "transfer-encoding"}
        for name, value in data["default_headers"].items():
            if not isinstance(name, str) or not name or any(ch in name for ch in "\r\n:"):
                raise ValueError(f"{case_id}: invalid default header name")
            if name.lower() in forbidden_headers:
                raise ValueError(f"{case_id}: default header {name} is controlled by the HTTP transport")
            if not isinstance(value, str) or any(ch in value for ch in "\r\n"):
                raise ValueError(f"{case_id}: invalid default header value for {name}")
        if expected_default_headers is None:
            expected_default_headers = data["default_headers"]
        elif data["default_headers"] != expected_default_headers:
            raise ValueError("all fixtures must use the same default_headers for the singleton Java HttpClient")
        if operation in POST_OPERATIONS:
            if not isinstance(data.get("api_body"), str) or not data["api_body"]:
                raise ValueError(f"{case_id}: api_body must be a non-empty string")
            if not isinstance(json.loads(data["api_body"]), dict):
                raise ValueError(f"{case_id}: api_body must be a JSON object string")
        elif operation in GET_OPERATIONS:
            if not isinstance(data.get("query_param"), str):
                raise ValueError(f"{case_id}: query_param must be a string")
        elif operation in DOWNLOAD_OPERATIONS:
            if not isinstance(data.get("media_id"), str) or not data["media_id"]:
                raise ValueError(f"{case_id}: media_id must be a non-empty string")
            if operation == "miniapp.http.media-download-file":
                if row.get("comparison") not in ("file_semantics", "http_error"):
                    raise ValueError(f"{case_id}: media-download-file requires file_semantics or http_error")
                if row["comparison"] == "file_semantics":
                    for key in ("expected_file_prefix", "expected_file_extension", "expected_bytes_hex"):
                        if not isinstance(data.get(key), str) or not data[key]:
                            raise ValueError(f"{case_id}: {key} must be a non-empty string")
                    if not data["expected_file_extension"].startswith("."):
                        raise ValueError(f"{case_id}: expected_file_extension must begin with .")
                    try:
                        bytes.fromhex(data["expected_bytes_hex"])
                    except ValueError as error:
                        raise ValueError(f"{case_id}: expected_bytes_hex invalid") from error

        elif operation in UPLOAD_OPERATIONS:
            if not isinstance(data.get("media_type"), str) or not data["media_type"]:
                raise ValueError(f"{case_id}: media_type must be a non-empty string")
            if not isinstance(data.get("file_name"), str) or not data["file_name"]:
                raise ValueError(f"{case_id}: file_name must be a non-empty string")
            if Path(data["file_name"]).name != data["file_name"] or data["file_name"] in {".", ".."} or any(ch in data["file_name"] for ch in "\r\n"):
                raise ValueError(f"{case_id}: file_name must be a safe basename")
            payload_hex = data.get("file_bytes_hex")
            if not isinstance(payload_hex, str) or len(payload_hex) % 2 or any(ch not in "0123456789abcdefABCDEF" for ch in payload_hex):
                raise ValueError(f"{case_id}: file_bytes_hex must be an even-length hexadecimal string")
            form_fields = data.get("form_fields", {})
            if not isinstance(form_fields, dict) or any(not isinstance(k, str) or not isinstance(v, str) for k, v in form_fields.items()):
                raise ValueError(f"{case_id}: form_fields must be a string map")
        if operation == "miniapp.http.get-custom-client-headers" and not data["default_headers"]:
            raise ValueError(f"{case_id}: custom-client-headers requires non-empty default_headers")
        if not data["token_path"].startswith("/") or not data["api_path"].startswith("/"):
            raise ValueError(f"{case_id}: token_path and api_path must be absolute paths")
        if not isinstance(data["token_responses"], list) or not 1 <= len(data["token_responses"]) <= 2:
            raise ValueError(f"{case_id}: token_responses must contain one or two responses")
        if not isinstance(data["api_responses"], list) or not 1 <= len(data["api_responses"]) <= 2:
            raise ValueError(f"{case_id}: api_responses must contain one or two responses")
        for index, response in enumerate(data["token_responses"]):
            status, body, _ = response_parts(response, f"{case_id}: token_responses[{index}]")
            if not 200 <= status < 300:
                continue
            value = json.loads(body)
            if not isinstance(value, dict) or not isinstance(value.get("access_token"), str):
                raise ValueError(f"{case_id}: token_responses[{index}] requires access_token")
        for index, response in enumerate(data["api_responses"]):
            status, body, _ = response_parts(response, f"{case_id}: api_responses[{index}]")
            if body == "":
                continue
            if operation in DOWNLOAD_OPERATIONS:
                response_object = data["api_responses"][index]
                if isinstance(response_object, dict):
                    response_headers = response_object.get("response_headers", {})
                    if not isinstance(response_headers, dict) or any(
                        not isinstance(name, str) or not isinstance(value, str)
                        for name, value in response_headers.items()
                    ):
                        raise ValueError(f"{case_id}: response_headers must be a string map")
                continue
            if operation == "miniapp.http.post-xml-response":
                if index != 0 or not body.startswith("<xml>"):
                    raise ValueError(f"{case_id}: XML response must start with <xml> and be the first response")
                continue
            value = json.loads(body)
            if not isinstance(value, dict) or not isinstance(value.get("errcode"), int):
                raise ValueError(f"{case_id}: api_responses[{index}] requires integer errcode")
        if operation in {"miniapp.http.post-xml-response", "miniapp.http.post-empty-response"}:
            if len(data["api_responses"]) != 1:
                raise ValueError(f"{case_id}: this POST response scenario requires exactly one API response")
    return rows


def run_step(command: list[str], cwd: Path, out: Path, label: str, env: dict, timeout: int = 900) -> dict:
    stdout = out / f"{label}.stdout.log"
    stderr = out / f"{label}.stderr.log"
    started = time.monotonic()
    with stdout.open("wb") as output, stderr.open("wb") as errors:
        result = subprocess.run(command, cwd=cwd, stdout=output, stderr=errors, env=env, timeout=timeout)
    step = {
        "step": label,
        "command": [str(value) for value in command],
        "cwd": str(cwd),
        "exit_code": result.returncode,
        "seconds": round(time.monotonic() - started, 3),
        "stdout": stdout.name,
        "stderr": stderr.name,
    }
    print(json.dumps({key: step[key] for key in ("step", "exit_code", "seconds", "stdout", "stderr")}), flush=True)
    if result.returncode:
        raise RuntimeError(f"{label}: exit {result.returncode}; see {stderr} and {stdout}")
    return step


def verify_java_provenance(log: str, expected_dirs: list[Path]) -> None:
    markers = ("ORACLE_COMMON_CLASS_SOURCE=", "ORACLE_MINIAPP_CLASS_SOURCE=")
    for marker, expected in zip(markers, expected_dirs):
        locations = [line[len(marker):] for line in log.splitlines() if line.startswith(marker)]
        if len(locations) != 1:
            raise ValueError(f"missing or duplicate Java class provenance: {marker}")
        uri = urlparse(locations[0])
        if uri.scheme != "file" or uri.netloc or uri.query or uri.fragment:
            raise ValueError(f"unexpected Java class URL: {locations[0]}")
        loaded = Path(url2pathname(uri.path)).resolve()
        if loaded != expected.resolve():
            raise ValueError(f"Java class loaded from {loaded}, expected {expected}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--java-root", type=Path, required=True)
    parser.add_argument("--jdk-home", type=Path, default=os.environ.get("JAVA_HOME"))
    parser.add_argument("--fixtures", type=Path, default=DEFAULT_FIXTURE)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    out = args.out.resolve()
    if out.exists() and any(out.iterdir()):
        print(f"Output directory must be empty: {out}", file=sys.stderr)
        return 2
    out.mkdir(parents=True, exist_ok=True)
    report = {
        "schema_version": 1,
        "passed": False,
        "status": "INFRA_ERROR",
        "oracle_commit": ORACLE,
        "compatibility_claim": "scenario_only",
        "steps": [],
    }
    exit_code = 2
    try:
        oracle = args.java_root.resolve()
        fixtures = args.fixtures.resolve()
        if subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=oracle, text=True).strip() != ORACLE:
            raise RuntimeError("Oracle HEAD does not match pinned WxJava SHA")
        if subprocess.check_output(
            ["git", "status", "--porcelain", "--untracked-files=no"], cwd=oracle, text=True
        ).strip():
            raise RuntimeError("Oracle tracked source is modified")
        cases = load_http_fixtures(fixtures)

        if args.jdk_home is None:
            raise RuntimeError("--jdk-home or JAVA_HOME is required")
        jdk = args.jdk_home.resolve()
        suffix = ".exe" if os.name == "nt" else ""
        java = jdk / "bin" / f"java{suffix}"
        javac = jdk / "bin" / f"javac{suffix}"
        if not java.is_file() or not javac.is_file():
            raise RuntimeError("JDK java/javac not found")

        env = os.environ.copy()
        env["JAVA_HOME"] = str(jdk)
        env["PATH"] = str(jdk / "bin") + os.pathsep + env.get("PATH", "")
        env["PYTHONIOENCODING"] = "utf-8"

        before = source_digest()
        report.update(
            rust_head=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
            rust_dirty=bool(subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT, text=True).strip()),
            fixtures_sha256=sha(fixtures),
            source_sha256=before,
            jdk=str(jdk),
            fixture_count=len(cases),
        )
        shutil.copyfile(fixtures, out / "fixtures.jsonl")
        report["steps"].append(run_step([str(java), "-version"], ROOT, out, "java-version", env))
        report["steps"].append(run_step(["cargo", "--version"], ROOT, out, "cargo-version", env))

        mvn = shutil.which("mvn", path=env["PATH"])
        if mvn is None:
            raise RuntimeError("Maven is not installed")
        report["steps"].append(run_step([
            mvn, "-f", str(oracle / "pom.xml"),
            "-pl", "weixin-java-miniapp", "-am",
            "-DskipTests", "-Dcheckstyle.skip=true", "-Dspotless.check.skip=true",
            "clean", "package", "dependency:build-classpath",
            "-Dmdep.outputFile=target/phase1-http-classpath.txt",
        ], oracle, out, "oracle-build", env, timeout=1800))

        common_classes = oracle / "weixin-java-common" / "target" / "classes"
        miniapp_classes = oracle / "weixin-java-miniapp" / "target" / "classes"
        class_dirs = [common_classes, miniapp_classes]
        classpath_file = oracle / "weixin-java-miniapp" / "target" / "phase1-http-classpath.txt"
        dependencies = classpath_file.read_text(encoding="utf-8-sig").strip()
        classpath = os.pathsep.join([str(path) for path in class_dirs] + [dependencies])

        java_classes = out / "java-classes"
        java_classes.mkdir()
        java_adapter = ROOT / "tools" / "compat" / "http" / "java" / "Phase1HttpOracle.java"
        report["steps"].append(run_step([
            str(javac), "--add-modules", "jdk.httpserver", "-encoding", "UTF-8",
            "-cp", classpath, "-d", str(java_classes), str(java_adapter),
        ], ROOT, out, "java-adapter-build", env))
        report["steps"].append(run_step([
            str(java), "--add-modules", "jdk.httpserver", "-Dfile.encoding=UTF-8",
            "-cp", str(java_classes) + os.pathsep + classpath,
            "Phase1HttpOracle", str(fixtures),
        ], ROOT, out, "java", env, timeout=90))

        report["steps"].append(run_step([
            "cargo", "build", "--offline", "--locked", "-p", "wx-rust-miniapp",
            "--example", "phase1_http_oracle", "--message-format=json",
        ], ROOT, out, "rust-build", env, timeout=1800))
        executables = []
        for line in (out / "rust-build.stdout.log").read_text(encoding="utf-8").splitlines():
            item = json.loads(line)
            if (item.get("reason") == "compiler-artifact"
                    and item.get("target", {}).get("name") == "phase1_http_oracle"
                    and item.get("executable")):
                executables.append(item["executable"])
        if len(set(executables)) != 1:
            raise RuntimeError("Cargo did not emit exactly one HTTP adapter executable")
        executable = Path(executables[0])
        report["steps"].append(run_step(
            [str(executable), str(fixtures)], ROOT, out, "rust", env, timeout=90
        ))

        java_provenance = (out / "java.stderr.log").read_text(encoding="utf-8", errors="replace")
        verify_java_provenance(java_provenance, class_dirs)
        report["java_provenance"] = java_provenance
        report["java_class_sha256"] = {
            "WxError": sha(common_classes / "me/chanjar/weixin/common/error/WxError.class"),
            "WxMaServiceImpl": sha(miniapp_classes / "cn/binarywang/wx/miniapp/api/impl/WxMaServiceImpl.class"),
        }
        report["rust_binary_sha256"] = sha(executable)

        result = compare_records(
            cases,
            load_jsonl(out / "java.stdout.log"),
            load_jsonl(out / "rust.stdout.log"),
        )
        report.update(result)
        if before != source_digest() or report["fixtures_sha256"] != sha(fixtures):
            raise RuntimeError("source or fixtures changed during verification; results are stale")
        if report["errors"]:
            report["status"], exit_code = "INFRA_ERROR", 2
        elif report["passed"]:
            report["status"], exit_code = "EQUIVALENT", 0
        else:
            report["status"], exit_code = "DIFFERENT", 1
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, TimeoutError) as error:
        report.update(passed=False, status="INFRA_ERROR", infrastructure_error=str(error))

    (out / "report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "report": str(out / "report.json"),
        "status": report["status"],
        "counts": report.get("counts", {}),
        "error": report.get("infrastructure_error"),
    }, ensure_ascii=False))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
