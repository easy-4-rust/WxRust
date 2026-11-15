#!/usr/bin/env python3
"""Build and execute the pinned Java SDK and actual Rust SDK on shared fixtures.

Exit codes: 0 = all scenarios equivalent, 1 = differences/unsupported,
2 = infrastructure, input, provenance or adapter protocol failure.
No expected-difference allowlist is applied.
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

from compare import compare_records, load_jsonl

ORACLE = 'a49d6e1461752c06b752d2afd8aeeb7e6e78cefe'
ROOT = Path(__file__).resolve().parents[2]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def capture(args, cwd):
    return subprocess.check_output(args, cwd=cwd, text=True, encoding='utf-8', errors='replace').strip()


def source_digest() -> str:
    paths = [ROOT/'Cargo.toml', ROOT/'Cargo.lock']
    for directory in ('crates/wx-rust-common', 'crates/wx-rust-pay', 'tools/compat'):
        paths += [p for p in (ROOT/directory).rglob('*')
                  if p.is_file() and p.suffix in ('.rs', '.toml', '.java', '.py', '.jsonl')
                  and 'target' not in p.parts and '__pycache__' not in p.parts]
    hasher = hashlib.sha256()
    for path in sorted(set(paths)):
        if path.exists():
            hasher.update(path.relative_to(ROOT).as_posix().encode())
            hasher.update(bytes.fromhex(sha(path)))
    return hasher.hexdigest()


def run_step(command, cwd: Path, out: Path, label: str, env: dict, timeout=600):
    stdout, stderr = out/(label+'.stdout.log'), out/(label+'.stderr.log')
    start = time.monotonic()
    with stdout.open('wb') as a, stderr.open('wb') as b:
        result = subprocess.run(command, cwd=cwd, stdout=a, stderr=b, env=env, timeout=timeout)
    info = {'step': label, 'command': [str(x) for x in command], 'cwd': str(cwd),
            'exit_code': result.returncode, 'seconds': round(time.monotonic()-start, 3),
            'stdout': stdout.name, 'stderr': stderr.name}
    print(json.dumps({key: info[key] for key in ('step', 'exit_code', 'seconds', 'stdout', 'stderr')}, ensure_ascii=False), flush=True)
    if result.returncode:
        raise RuntimeError(f'{label}: exit {result.returncode}; see {stderr.name} and {stdout.name}')
    return info


def verify_java_provenance(log: str, expected_dirs: list[Path]) -> None:
    """Fail if an SDK class was loaded from an unpinned dependency JAR."""
    for marker, expected in zip(('ORACLE_CLASS_SOURCE=', 'PAY_CLASS_SOURCE='), expected_dirs):
        locations = [line[len(marker):] for line in log.splitlines() if line.startswith(marker)]
        if len(locations) != 1:
            raise ValueError('missing or duplicate Java class provenance: '+marker)
        url = urlparse(locations[0])
        if url.scheme != 'file' or url.netloc or url.query or url.fragment:
            raise ValueError('unexpected Java class URL: '+locations[0])
        loaded = Path(url2pathname(url.path)).resolve()
        if loaded != expected.resolve():
            raise ValueError(f'Java class loaded from {loaded}, expected {expected}')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--java-root', type=Path, required=True)
    parser.add_argument('--jdk-home', type=Path, default=os.environ.get('JAVA_HOME'))
    parser.add_argument('--fixtures', type=Path, default=ROOT/'tools/compat/fixtures.jsonl')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    # Never overwrite a previous run's evidence.
    if out.exists() and any(out.iterdir()):
        print('Output directory must be empty: '+str(out), file=sys.stderr)
        return 2
    out.mkdir(parents=True, exist_ok=True)
    report = {'schema_version': 1, 'passed': False, 'status': 'INFRA_ERROR',
              'oracle_commit': ORACLE, 'compatibility_claim': 'scenario_only', 'steps': []}
    exit_code = 2
    try:
        oracle, fixtures = args.java_root.resolve(), args.fixtures.resolve()
        if capture(['git', 'rev-parse', 'HEAD'], oracle) != ORACLE:
            raise RuntimeError('Oracle HEAD does not match pinned SHA')
        if capture(['git', 'status', '--porcelain', '--untracked-files=no'], oracle):
            raise RuntimeError('Oracle tracked source is modified')
        cases = load_jsonl(fixtures)
        if not cases or len({row.get('id') for row in cases}) != len(cases):
            raise ValueError('non-empty fixtures with unique ids required')
        for row in cases:
            if not isinstance(row.get('id'), str) or not isinstance(row.get('operation'), str) or not isinstance(row.get('input'), dict):
                raise ValueError('invalid fixture envelope')
        env = os.environ.copy()
        if args.jdk_home is None:
            raise RuntimeError('--jdk-home or JAVA_HOME is required')
        jdk = args.jdk_home.resolve()
        suffix = '.exe' if os.name == 'nt' else ''
        java, javac = jdk/'bin'/('java'+suffix), jdk/'bin'/('javac'+suffix)
        if not java.is_file() or not javac.is_file():
            raise RuntimeError('JDK java/javac not found')
        env['JAVA_HOME'] = str(jdk)
        env['PATH'] = str(jdk/'bin')+os.pathsep+env.get('PATH', '')
        env['PYTHONIOENCODING'] = 'utf-8'
        before = source_digest()
        report.update(rust_head=capture(['git', 'rev-parse', 'HEAD'], ROOT),
                      rust_dirty=bool(capture(['git', 'status', '--porcelain'], ROOT)),
                      fixtures_sha256=sha(fixtures), source_sha256=before,
                      jdk=str(jdk), fixture_count=len(cases))
        shutil.copyfile(fixtures, out/'fixtures.jsonl')
        report['steps'].append(run_step([str(java), '-version'], ROOT, out, 'java-version', env))
        report['steps'].append(run_step(['cargo', '--version'], ROOT, out, 'cargo-version', env))
        mvn = shutil.which('mvn', path=env['PATH'])
        if mvn is None:
            raise RuntimeError('Maven is not installed')
        report['steps'].append(run_step([
            mvn, '-f', str(oracle/'pom.xml'), '-pl', 'weixin-java-common,weixin-java-pay', '-am',
            '-DskipTests', '-Dcheckstyle.skip=true', '-Dspotless.check.skip=true',
            'clean', 'package', 'dependency:build-classpath', '-Dmdep.outputFile=target/phase1-classpath.txt'],
            oracle, out, 'oracle-build', env))
        class_dirs = [oracle/'weixin-java-common/target/classes', oracle/'weixin-java-pay/target/classes']
        dependencies = (oracle/'weixin-java-pay/target/phase1-classpath.txt').read_text(encoding='utf-8-sig').strip()
        classpath = os.pathsep.join([str(p) for p in class_dirs]+[dependencies])
        classes = out/'java-classes'
        classes.mkdir()
        report['steps'].append(run_step([str(javac), '-encoding', 'UTF-8', '-cp', classpath,
            '-d', str(classes), str(ROOT/'tools/compat/java/Phase1Oracle.java')], ROOT, out, 'java-adapter-build', env))
        report['steps'].append(run_step([str(java), '-Dfile.encoding=UTF-8', '-cp', str(classes)+os.pathsep+classpath,
            'Phase1Oracle', str(fixtures)], ROOT, out, 'java', env))
        report['steps'].append(run_step(['cargo', 'build', '--offline', '--locked', '-p', 'wx-rust-pay',
            '--example', 'phase1_oracle', '--message-format=json'], ROOT, out, 'rust-build', env))
        executables = []
        for line in (out/'rust-build.stdout.log').read_text(encoding='utf-8').splitlines():
            row = json.loads(line)
            if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'phase1_oracle' and row.get('executable'):
                executables.append(row['executable'])
        if len(set(executables)) != 1:
            raise RuntimeError('Cargo did not emit exactly one adapter executable')
        executable = Path(executables[0])
        report['steps'].append(run_step([str(executable), str(fixtures)], ROOT, out, 'rust', env))
        # Require pinned source dirs in actual Java class loading evidence.
        provenance = (out/'java.stderr.log').read_text(encoding='utf-8', errors='replace')
        verify_java_provenance(provenance, class_dirs)
        report['java_provenance'] = provenance
        report['rust_binary_sha256'] = sha(executable)
        report['java_class_sha256'] = {
            'WxError': sha(class_dirs[0]/'me/chanjar/weixin/common/error/WxError.class'),
            'SignUtils': sha(class_dirs[1]/'com/github/binarywang/wxpay/util/SignUtils.class')}
        result = compare_records(cases, load_jsonl(out/'java.stdout.log'), load_jsonl(out/'rust.stdout.log'))
        report.update(result)
        if before != source_digest() or report['fixtures_sha256'] != sha(fixtures):
            raise RuntimeError('source or fixtures changed during verification; results are stale')
        if report['errors']:
            report['status'], exit_code = 'INFRA_ERROR', 2
        elif report['passed']:
            report['status'], exit_code = 'EQUIVALENT', 0
        else:
            report['status'], exit_code = 'DIFFERENT', 1
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        report.update(passed=False, status='INFRA_ERROR', infrastructure_error=str(error))
    (out/'report.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({'report': str(out/'report.json'), 'status': report['status'],
                      'counts': report.get('counts', {}), 'error': report.get('infrastructure_error')}, ensure_ascii=False))
    return exit_code


if __name__ == '__main__':
    raise SystemExit(main())
