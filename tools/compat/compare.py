"""Strict, fail-closed comparison for the WP-03 JSONL adapter protocol."""
from __future__ import annotations

from collections import Counter
import json
from pathlib import Path
from typing import Any
import xml.etree.ElementTree as ET


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    def reject_constant(value: str):
        raise ValueError(f'non-JSON constant: {value}')

    def unique_object(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f'duplicate JSON member: {key}')
            result[key] = value
        return result

    rows = []
    for line_number, line in enumerate(path.read_text(encoding='utf-8-sig').splitlines(), 1):
        if not line.strip():
            continue
        try:
            row = json.loads(line, parse_constant=reject_constant, object_pairs_hook=unique_object)
            if not isinstance(row, dict):
                raise ValueError('record must be an object')
            rows.append(row)
        except ValueError as error:
            raise ValueError(f'{path.name}:{line_number}: {error}') from error
    return rows


def _strict(value: Any):
    """JSON object order is irrelevant; primitive types and array order are not."""
    if isinstance(value, dict):
        return ('object', tuple((key, _strict(item)) for key, item in sorted(value.items())))
    if isinstance(value, list):
        return ('array', tuple(_strict(item) for item in value))
    return (type(value).__name__, value)


def _xml(value: Any):
    if not isinstance(value, str):
        raise ValueError('XML comparison requires a string value')
    if '<!DOCTYPE' in value.upper() or '<!ENTITY' in value.upper():
        raise ValueError('DTD/entity declarations are not allowed')
    def element(node):
        return (node.tag, tuple(sorted(node.attrib.items())), node.text,
                tuple((element(child), child.tail) for child in node))
    return element(ET.fromstring(value))



def _file_semantics(fixture: dict, java: dict, rust: dict) -> bool:
    """Compare explicit file-contract invariants, retaining raw names and traces.

    This intentionally does NOT establish byte-for-byte result equality: both SDKs
    allocate independent random temporary names. Any other difference remains a
    failure, including a changed HTTP request/response, file bytes or metadata.
    """
    expected = fixture['input']
    required = ('expected_file_prefix', 'expected_file_extension', 'expected_bytes_hex')
    for key in required:
        if not isinstance(expected.get(key), str) or not expected[key]:
            raise ValueError(f'file_semantics requires {key}')
    prefix = expected['expected_file_prefix']
    extension = expected['expected_file_extension']
    if not extension.startswith('.') or '/' in prefix or '\\' in prefix:
        raise ValueError('file_semantics invalid expected filename contract')
    try:
        desired_bytes = bytes.fromhex(expected['expected_bytes_hex'])
    except ValueError as error:
        raise ValueError('file_semantics invalid expected_bytes_hex') from error

    results = []
    for entry in (java, rust):
        if entry['kind'] != 'value':
            return False
        value = entry.get('value')
        if not isinstance(value, dict):
            raise ValueError('file_semantics requires value object')
        for key in ('response_body', 'request_trace', 'token_calls', 'api_calls'):
            if key not in value:
                raise ValueError(f'file_semantics missing {key}')
        if not isinstance(value['request_trace'], list):
            raise ValueError('file_semantics requires request_trace')
        if type(value['token_calls']) is not int or type(value['api_calls']) is not int:
            raise ValueError('file_semantics requires integer request counts')
        if value['token_calls'] < 1 or value['api_calls'] != 1:
            raise ValueError('file_semantics requires one API request and token evidence')

        if not isinstance(value['response_body'], str):
            raise ValueError('file_semantics requires JSON response_body')
        try:
            file = json.loads(value['response_body'])
        except (ValueError, TypeError) as error:
            raise ValueError('file_semantics requires JSON file result') from error
        if not isinstance(file, dict) or set(file) != {
            'file_name', 'temp_dir_name', 'file_exists', 'bytes_hex',
        }:
            raise ValueError('file_semantics requires exact file evidence fields')
        name, directory = file['file_name'], file['temp_dir_name']
        if not isinstance(name, str) or not isinstance(directory, str):
            raise ValueError('file_semantics filename and directory must be strings')
        try:
            content = bytes.fromhex(file['bytes_hex'])
        except (TypeError, ValueError) as error:
            raise ValueError('file_semantics invalid downloaded bytes_hex') from error

        valid = (
            file['file_exists'] is True
            and name.startswith(prefix)
            and name.endswith(extension)
            and len(name) > len(prefix) + len(extension)
            and '/' not in name and '\\' not in name
            and directory.startswith('wxma')
            and len(directory) > 4
            and '/' not in directory and '\\' not in directory
            and content == desired_bytes
        )
        results.append((valid, value))

    left, right = results
    return (
        left[0] and right[0]
        and _strict(left[1]['request_trace']) == _strict(right[1]['request_trace'])
        and left[1]['token_calls'] == right[1]['token_calls']
        and left[1]['api_calls'] == right[1]['api_calls']
    )

def compare_records(fixtures: list[dict], java: list[dict], rust: list[dict]) -> dict:
    errors: list[str] = []
    def index(rows, label):
        result = {}
        for row in rows:
            key = row.get('id') if isinstance(row, dict) else None
            if not isinstance(key, str) or not key:
                errors.append(f'{label}: missing/non-string id')
            elif key in result:
                errors.append(f'{label}: duplicate id {key}')
            else:
                result[key] = row
        return result

    expected = index(fixtures, 'fixtures')
    left, right = index(java, 'java'), index(rust, 'rust')
    if not fixtures:
        errors.append('empty fixture set')
    for label, rows in [('java', left), ('rust', right)]:
        if set(rows) != set(expected):
            errors.append(f'{label}: missing={sorted(set(expected)-set(rows))}, unexpected={sorted(set(rows)-set(expected))}')
    cases = []
    for key, fixture in expected.items():
        item = {'id': key, 'operation': fixture.get('operation'), 'status': 'INFRA_ERROR'}
        try:
            if not isinstance(fixture.get('operation'), str) or not fixture['operation']:
                raise ValueError('fixture requires a non-empty operation')
            if not isinstance(fixture.get('input'), dict):
                raise ValueError('fixture requires an input object')
            if key not in left or key not in right:
                raise ValueError('adapter output missing')
            for row in (left[key], right[key]):
                if row.get('operation') != fixture.get('operation'):
                    raise ValueError('operation mismatch')
                if not isinstance(row.get('result'), dict):
                    raise ValueError('missing result object')
                if row['result'].get('kind') not in ('value', 'exception', 'unsupported', 'adapter_error'):
                    raise ValueError('unrecognized result kind')
                if row['result']['kind'] == 'value' and 'value' not in row['result']:
                    raise ValueError('value result requires value member')
                if row['result']['kind'] == 'exception' and not isinstance(row['result'].get('type'), str):
                    raise ValueError('exception result requires a type')
            a, b = left[key]['result'], right[key]['result']
            item.update(java=a, rust=b)
            kinds = {a['kind'], b['kind']}
            if 'adapter_error' in kinds:
                raise ValueError('adapter reported an infrastructure error')
            if 'unsupported' in kinds:
                item['status'] = 'UNSUPPORTED'
            else:
                mode = fixture.get('comparison', 'json')
                if mode == 'json':
                    equal = _strict(a) == _strict(b)
                elif mode == 'xml' and kinds == {'value'}:
                    equal = _xml(a['value']) == _xml(b['value'])
                elif mode == 'xml':
                    equal = _strict(a) == _strict(b)
                elif mode == 'error_category':
                    # Cross-language exception *category* parity is intentionally
                    # weaker than exact type/message parity. Do not apply it to
                    # existing non-fallible production APIs without an explicit fixture.
                    if kinds == {'exception'}:
                        allowed_categories = {'syntax_error', 'null_root', 'field_error'}
                        for result in (a, b):
                            if result.get('category') not in allowed_categories:
                                raise ValueError('error_category requires a recognized actual exception category')
                        equal = a['category'] == b['category']
                    else:
                        equal = False
                elif mode == 'http_error':
                    # Compare real HTTP failure category and server-observed wire trace,
                    # while retaining the original Java/Rust exception type and message.
                    if kinds == {'exception'}:
                        allowed_categories = {'http_status', 'empty_response_runtime', 'empty_response_business', 'business_error', 'missing_content_disposition'}
                        for result in (a, b):
                            if result.get('category') not in allowed_categories:
                                raise ValueError('http_error requires a recognized HTTP exception category')
                            if not isinstance(result.get('request_trace'), list):
                                raise ValueError('http_error requires request_trace')
                            if not isinstance(result.get('token_calls'), int) or not isinstance(result.get('api_calls'), int):
                                raise ValueError('http_error requires token_calls and api_calls')
                        error_codes_equal = (
                            a.get('error_code') == b.get('error_code')
                            and (a.get('error_code') is None or isinstance(a.get('error_code'), int))
                            and (b.get('error_code') is None or isinstance(b.get('error_code'), int))
                        )
                        equal = (
                            a['category'] == b['category']
                            and error_codes_equal
                            and _strict(a['request_trace']) == _strict(b['request_trace'])
                            and a['token_calls'] == b['token_calls']
                            and a['api_calls'] == b['api_calls']
                        )
                    else:
                        equal = False
                elif mode == 'file_semantics':
                    item['comparison_scope'] = 'file_semantics'
                    item['raw_value_equal'] = _strict(a) == _strict(b)
                    equal = _file_semantics(fixture, a, b)
                else:
                    raise ValueError(f'unsupported comparison mode: {mode}')
                item['status'] = 'EQUIVALENT' if equal else 'DIFFERENT'
        except (ValueError, ET.ParseError) as error:
            item['reason'] = str(error)
            errors.append(f'{key}: {error}')
        cases.append(item)
    counts = Counter(item['status'] for item in cases)
    return {'schema_version': 1, 'passed': bool(cases) and not errors and counts['EQUIVALENT'] == len(cases),
            'counts': dict(counts), 'case_count': len(cases), 'errors': errors, 'cases': cases}
