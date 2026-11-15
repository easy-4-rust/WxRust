"""Regression tests for the fail-closed differential comparator."""
import copy
import importlib.util
from pathlib import Path
import unittest


class ComparatorTests(unittest.TestCase):
    def setUp(self):
        path = Path(__file__).with_name('compare.py')
        self.assertTrue(path.is_file(), 'WP-03 comparator is not implemented yet')
        spec = importlib.util.spec_from_file_location('compat_compare', path)
        self.api = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.api)

    def compare(self, left, right, mode='json'):
        fixture = {'id': 'case1', 'operation': 'test', 'comparison': mode, 'input': {}}
        a = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'value', 'value': left}}
        b = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'value', 'value': right}}
        return self.api.compare_records([fixture], [a], [b])

    def test_json_object_order_is_semantic(self):
        self.assertTrue(self.compare({'a': 1, 'b': 2}, {'b': 2, 'a': 1})['passed'])

    def test_missing_null_and_empty_are_not_collapsed(self):
        for left, right in [({}, {'a': None}), ({'a': None}, {'a': ''})]:
            with self.subTest(left=left):
                self.assertEqual(self.compare(left, right)['cases'][0]['status'], 'DIFFERENT')

    def test_boolean_is_not_numeric_one(self):
        self.assertFalse(self.compare({'a': True}, {'a': 1})['passed'])

    def test_large_integers_are_not_rounded(self):
        self.assertFalse(self.compare(9007199254740992, 9007199254740993)['passed'])

    def test_array_order_is_preserved(self):
        self.assertFalse(self.compare([1, 2], [2, 1])['passed'])

    def test_xml_attribute_order_only_is_ignored(self):
        self.assertTrue(self.compare('<x a="1" b="2"/>', '<x b="2" a="1"/>', 'xml')['passed'])
        self.assertFalse(self.compare('<x> </x>', '<x/>', 'xml')['passed'])
        self.assertFalse(self.compare('<x><a/><b/></x>', '<x><b/><a/></x>', 'xml')['passed'])

    def test_xml_doctype_is_infrastructure_error_not_pass(self):
        report = self.compare('<!DOCTYPE x><x/>', '<!DOCTYPE x><x/>', 'xml')
        self.assertFalse(report['passed'])
        self.assertEqual(report['cases'][0]['status'], 'INFRA_ERROR')

    def test_empty_fixture_set_cannot_pass(self):
        report = self.api.compare_records([], [], [])
        self.assertFalse(report['passed'])
        self.assertTrue(report['errors'])

    def test_missing_duplicate_and_unexpected_outputs_fail_closed(self):
        fixture = {'id': 'case1', 'operation': 'test', 'input': {}}
        row = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'value', 'value': 1}}
        for rows in [[], [row, row], [dict(row, id='unexpected')]]:
            with self.subTest(rows=rows):
                report = self.api.compare_records([fixture], [row], rows)
                self.assertFalse(report['passed'])
                self.assertTrue(report['errors'])

    def test_unsupported_is_never_equivalent(self):
        fixture = {'id': 'case1', 'operation': 'test', 'input': {}}
        row = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'unsupported', 'reason': 'native SDK missing'}}
        report = self.api.compare_records([fixture], [row], [copy.deepcopy(row)])
        self.assertFalse(report['passed'])
        self.assertEqual(report['cases'][0]['status'], 'UNSUPPORTED')

    def test_exceptions_are_not_silently_coerced_to_values(self):
        fixture = {'id': 'case1', 'operation': 'test', 'input': {}}
        a = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'exception', 'type': 'ParseError'}}
        b = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'value', 'value': {'error_code': -99}}}
        self.assertEqual(self.api.compare_records([fixture], [a], [b])['cases'][0]['status'], 'DIFFERENT')

    def test_missing_operation_cannot_pass(self):
        fixture = {'id': 'case1', 'input': {}}
        row = {'id': 'case1', 'result': {'kind': 'value', 'value': 1}}
        report = self.api.compare_records([fixture], [row], [row])
        self.assertFalse(report['passed'])
        self.assertTrue(report['errors'])

    def test_exception_requires_a_type(self):
        fixture = {'id': 'case1', 'operation': 'test', 'input': {}}
        row = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'exception'}}
        report = self.api.compare_records([fixture], [row], [row])
        self.assertFalse(report['passed'])
        self.assertEqual(report['cases'][0]['status'], 'INFRA_ERROR')

    def test_error_category_mode_compares_only_declared_error_category(self):
        fixture = {'id': 'case1', 'operation': 'test', 'comparison': 'error_category', 'input': {}}
        java = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'com.google.gson.JsonSyntaxException',
            'message': 'Gson parse failed', 'category': 'syntax_error'}}
        rust = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'WxErrorParseError::MalformedJson',
            'message': 'Rust parse failed', 'category': 'syntax_error'}}
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertTrue(report['passed'])
        self.assertEqual(report['cases'][0]['java']['type'], 'com.google.gson.JsonSyntaxException')
        self.assertEqual(report['cases'][0]['rust']['type'], 'WxErrorParseError::MalformedJson')

    def test_error_category_mode_rejects_mismatched_category_and_success_values(self):
        fixture = {'id': 'case1', 'operation': 'test', 'comparison': 'error_category', 'input': {}}
        java = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'JsonSyntaxException', 'category': 'syntax_error'}}
        mismatched = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'WxErrorParseError::NullRoot', 'category': 'null_root'}}
        value = {'id': 'case1', 'operation': 'test', 'result': {'kind': 'value', 'value': -99}}
        for rust in [mismatched, value]:
            with self.subTest(rust=rust):
                report = self.api.compare_records([fixture], [java], [rust])
                self.assertFalse(report['passed'])
                self.assertEqual(report['cases'][0]['status'], 'DIFFERENT')

    def test_error_category_requires_actual_category_from_both_adapters(self):
        fixture = {'id': 'case1', 'operation': 'test', 'comparison': 'error_category', 'input': {}}
        java = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'JsonSyntaxException', 'category': 'syntax_error'}}
        rust = {'id': 'case1', 'operation': 'test', 'result': {
            'kind': 'exception', 'type': 'WxErrorParseError'}}
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertFalse(report['passed'])
        self.assertEqual(report['cases'][0]['status'], 'INFRA_ERROR')


    def test_file_semantics_ignores_only_random_names_and_preserves_raw_values(self):
        import json
        operation = "miniapp.http.media-download-file"
        fixture = {
            'id': 'case1', 'operation': operation, 'comparison': 'file_semantics',
            'input': {
                'expected_file_prefix': 'phase1-download',
                'expected_file_extension': '.bin',
                'expected_bytes_hex': '000102030d0a4142437f',
            }
        }
        trace = [
            {'kind': 'token', 'method': 'GET', 'target': '/cgi-bin/token', 'response_status': 200},
            {'kind': 'api', 'method': 'GET', 'target': '/cgi-bin/media/get?media_id=M1',
             'response_status': 200, 'response_body_hex': '000102030d0a4142437f'},
        ]
        def make_row(file_name, directory, *, payload='000102030d0a4142437f', actual_trace=trace, exists=True):
            return {'id': 'case1', 'operation': operation, 'result': {
                'kind': 'value',
                'value': {
                    'response_body': json.dumps({
                        'file_name': file_name, 'bytes_hex': payload,
                        'file_exists': exists, 'temp_dir_name': directory,
                    }),
                    'request_trace': actual_trace,
                    'token_calls': 1,
                    'api_calls': 1,
                },
            }}
        java = make_row('phase1-download123456.bin', 'wxma1234567')
        rust = make_row('phase1-download987654.bin', 'wxma9012-345')
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertTrue(report['passed'])
        self.assertEqual(report['cases'][0]['status'], 'EQUIVALENT')
        self.assertEqual(report['cases'][0]['comparison_scope'], 'file_semantics')
        self.assertFalse(report['cases'][0]['raw_value_equal'])
        self.assertNotEqual(
            json.loads(report['cases'][0]['java']['value']['response_body'])['file_name'],
            json.loads(report['cases'][0]['rust']['value']['response_body'])['file_name'],
        )

        for bad in [
            make_row('wrong123.bin', 'wxma2'),
            make_row('phase1-download123.txt', 'wxma2'),
            make_row('phase1-download123.bin', 'wrongdir'),
            make_row('phase1-download123.bin', 'wxma2', payload='ffff'),
            make_row('phase1-download123.bin', 'wxma2', exists=False),
            make_row('phase1-download123.bin', 'wxma2',
                     actual_trace=[dict(trace[0]), {**trace[1], 'response_status': 503}]),
        ]:
            with self.subTest(bad=bad):
                result = self.api.compare_records([fixture], [java], [bad])
                self.assertFalse(result['passed'])
                self.assertNotEqual(result['cases'][0]['status'], 'EQUIVALENT')

    def test_file_semantics_missing_required_evidence_fails_closed(self):
        import json
        fixture = {'id': 'case1', 'operation': 'miniapp.http.media-download-file',
                   'comparison': 'file_semantics', 'input': {}}
        record = {'id': 'case1', 'operation': 'miniapp.http.media-download-file',
                  'result': {'kind': 'value', 'value': {
                      'response_body': json.dumps({
                          'file_name':'a.bin','bytes_hex':'00',
                          'file_exists':True,'temp_dir_name':'wxma1'
                      }),
                      'request_trace': [], 'token_calls': 1, 'api_calls': 1
                  }}}
        result = self.api.compare_records([fixture], [record], [record])
        self.assertFalse(result['passed'])
        self.assertEqual(result['cases'][0]['status'], 'INFRA_ERROR')

    def test_http_error_mode_compares_category_and_wire_trace_but_preserves_raw_types(self):
        fixture = {"id": "case1", "operation": "http", "comparison": "http_error", "input": {}}
        trace = [{
            "sequence": 1, "kind": "api", "method": "GET",
            "target": "/probe?access_token=T1", "content_type": None,
            "body": "", "response_status": 503, "response_body": "{}"
        }]
        java = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "WxRuntimeException",
            "cause_type": "HttpResponseException", "category": "http_status",
            "request_trace": trace, "token_calls": 1, "api_calls": 1
        }}
        rust = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "WxErrorException::Runtime",
            "category": "http_status", "request_trace": trace,
            "token_calls": 1, "api_calls": 1
        }}
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertTrue(report["passed"])
        self.assertEqual(report["cases"][0]["java"]["type"], "WxRuntimeException")
        self.assertEqual(report["cases"][0]["rust"]["type"], "WxErrorException::Runtime")

    def test_http_error_mode_rejects_trace_status_or_category_mismatch(self):
        fixture = {"id": "case1", "operation": "http", "comparison": "http_error", "input": {}}
        trace = [{
            "sequence": 1, "kind": "api", "method": "GET",
            "target": "/probe", "content_type": None, "body": "",
            "response_status": 503, "response_body": "{}"
        }]
        java = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "JavaException", "category": "http_status",
            "request_trace": trace, "token_calls": 1, "api_calls": 1
        }}
        variants = [
            {"kind": "exception", "type": "RustException", "category": "empty_response_runtime",
             "request_trace": trace, "token_calls": 1, "api_calls": 1},
            {"kind": "exception", "type": "RustException", "category": "http_status",
             "request_trace": [{**trace[0], "response_status": 200}], "token_calls": 1, "api_calls": 1},
            {"kind": "value", "value": "not an exception"}
        ]
        for result in variants:
            rust = {"id": "case1", "operation": "http", "result": result}
            report = self.api.compare_records([fixture], [java], [rust])
            self.assertFalse(report["passed"])
            self.assertEqual(report["cases"][0]["status"], "DIFFERENT")

    def test_http_error_mode_compares_business_error_code(self):
        fixture = {"id": "case1", "operation": "http", "comparison": "http_error", "input": {}}
        trace = [{
            "sequence": 1, "kind": "api", "method": "POST", "target": "/probe",
            "content_type": "application/json; charset=UTF-8", "custom_headers": {},
            "body": "{}", "response_status": 200, "response_content_type": "text/xml; charset=UTF-8",
            "response_body": ""
        }]
        java = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "WxErrorException", "category": "empty_response_business",
            "error_code": -99, "request_trace": trace, "token_calls": 1, "api_calls": 1
        }}
        rust = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "WxErrorException::Wx", "category": "empty_response_business",
            "error_code": -99, "request_trace": trace, "token_calls": 1, "api_calls": 1
        }}
        self.assertTrue(self.api.compare_records([fixture], [java], [rust])["passed"])
        rust["result"]["error_code"] = -1
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertFalse(report["passed"])
        self.assertEqual(report["cases"][0]["status"], "DIFFERENT")
        rust["result"].pop("error_code")
        report = self.api.compare_records([fixture], [java], [rust])
        self.assertFalse(report["passed"])
        self.assertEqual(report["cases"][0]["status"], "DIFFERENT")

    def test_http_error_mode_requires_observed_trace_and_call_counts(self):
        fixture = {"id": "case1", "operation": "http", "comparison": "http_error", "input": {}}
        row = {"id": "case1", "operation": "http", "result": {
            "kind": "exception", "type": "SomeException", "category": "http_status"
        }}
        report = self.api.compare_records([fixture], [row], [row])
        self.assertFalse(report["passed"])
        self.assertEqual(report["cases"][0]["status"], "INFRA_ERROR")

    def test_deliberate_output_mutation_is_detected(self):
        self.assertEqual(self.compare({'signature': 'ABC'}, {'signature': 'ABD'})['cases'][0]['status'], 'DIFFERENT')


if __name__ == '__main__':
    unittest.main()
