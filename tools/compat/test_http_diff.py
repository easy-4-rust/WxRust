"""Fail-closed input validation for the isolated real HTTP differential runner."""
import importlib.util
from pathlib import Path
import tempfile
import unittest


class HttpRunnerInputTests(unittest.TestCase):
    def setUp(self):
        path = Path(__file__).with_name("run_http_diff.py")
        self.assertTrue(path.is_file(), "HTTP differential runner must exist")
        spec = importlib.util.spec_from_file_location("compat_http_runner", path)
        self.api = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.api)

    def write_jsonl(self, rows):
        folder = tempfile.TemporaryDirectory()
        self.addCleanup(folder.cleanup)
        path = Path(folder.name) / "fixtures.jsonl"
        path.write_text("\n".join(self.api.json.dumps(row) for row in rows) + "\n", encoding="utf-8")
        return path

    def valid_fixture(self):
        return {
            "id": "http-post-replay",
            "operation": "miniapp.http.post-replay",
            "input": {
                "app_id": "fixture-app",
                "app_secret": "fixture-secret",
                "token_path": "/cgi-bin/token",
                "api_path": "/cgi-bin/compat/http-probe",
                "api_query": "trace=wp03",
                "default_headers": {"X-WxRust-Probe": "phase1", "X-WxRust-Trace": "wp03"},
                "api_body": "{\"probe\":\"http-post-replay\"}",
                "token_responses": [
                    "{\"access_token\":\"T1\",\"expires_in\":7200}",
                    "{\"access_token\":\"T2\",\"expires_in\":7200}"
                ],
                "api_responses": [
                    "{\"errcode\":40001,\"errmsg\":\"invalid credential\"}",
                    "{\"errcode\":0,\"errmsg\":\"ok\",\"data\":{\"accepted\":true}}"
                ]
            }
        }

    def test_loads_the_shared_loopback_replay_fixture(self):
        rows = self.api.load_http_fixtures(self.write_jsonl([self.valid_fixture()]))
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["operation"], "miniapp.http.post-replay")
        self.assertEqual(len(rows[0]["input"]["token_responses"]), 2)
        self.assertEqual(len(rows[0]["input"]["api_responses"]), 2)

    def test_loads_get_query_order_fixture_without_post_body(self):
        fixture = self.valid_fixture()
        fixture["id"] = "http-get-query-order"
        fixture["operation"] = "miniapp.http.get-query-order"
        fixture["input"].pop("api_body")
        fixture["input"]["api_query"] = "base=1"
        fixture["input"]["query_param"] = "trace=wp03&scene=get"
        fixture["input"]["token_responses"] = [fixture["input"]["token_responses"][0]]
        fixture["input"]["api_responses"] = [fixture["input"]["api_responses"][1]]
        rows = self.api.load_http_fixtures(self.write_jsonl([fixture]))
        self.assertEqual(rows[0]["operation"], "miniapp.http.get-query-order")

    def test_rejects_get_without_query_param(self):
        fixture = self.valid_fixture()
        fixture["operation"] = "miniapp.http.get-query-order"
        fixture["input"].pop("api_body")
        with self.assertRaisesRegex(ValueError, "query_param"):
            self.api.load_http_fixtures(self.write_jsonl([fixture]))

    def test_accepts_non_2xx_and_empty_http_response_fixtures(self):
        non_2xx = self.valid_fixture()
        non_2xx["id"] = "http-get-non2xx"
        non_2xx["operation"] = "miniapp.http.get-query-order"
        non_2xx["comparison"] = "http_error"
        non_2xx["input"].pop("api_body")
        non_2xx["input"]["query_param"] = "trace=wp03&scene=non2xx"
        non_2xx["input"]["token_responses"] = [non_2xx["input"]["token_responses"][0]]
        non_2xx["input"]["api_responses"] = [{
            "status": 503, "body": "{\"errcode\":0,\"errmsg\":\"upstream unavailable\"}"
        }]

        empty = self.valid_fixture()
        empty["id"] = "http-get-empty-body"
        empty["operation"] = "miniapp.http.get-query-order"
        empty["comparison"] = "http_error"
        empty["input"].pop("api_body")
        empty["input"]["query_param"] = "trace=wp03&scene=empty"
        empty["input"]["token_responses"] = [empty["input"]["token_responses"][0]]
        empty["input"]["api_responses"] = [{"status": 200, "body": ""}]

        rows = self.api.load_http_fixtures(self.write_jsonl([non_2xx, empty]))
        self.assertEqual(len(rows), 2)

    def test_loads_custom_client_headers_fixture(self):
        fixture = self.valid_fixture()
        fixture["id"] = "http-get-custom-client-headers"
        fixture["operation"] = "miniapp.http.get-custom-client-headers"
        fixture["input"].pop("api_body")
        fixture["input"]["token_path"] = "/cgi-bin/token-custom-headers"
        fixture["input"]["query_param"] = "trace=wp03&scene=custom-headers"
        fixture["input"]["token_responses"] = [fixture["input"]["token_responses"][0]]
        fixture["input"]["api_responses"] = [fixture["input"]["api_responses"][1]]
        rows = self.api.load_http_fixtures(self.write_jsonl([fixture]))
        self.assertEqual(rows[0]["operation"], "miniapp.http.get-custom-client-headers")
        self.assertEqual(rows[0]["input"]["default_headers"]["X-WxRust-Probe"], "phase1")

    def test_loads_xml_and_empty_simple_post_response_fixtures(self):
        xml = self.valid_fixture()
        xml["id"] = "http-post-xml-response"
        xml["operation"] = "miniapp.http.post-xml-response"
        xml["input"]["api_body"] = "{\"probe\":\"xml-response\"}"
        xml["input"]["token_responses"] = [xml["input"]["token_responses"][0]]
        xml["input"]["api_responses"] = [{
            "status": 200,
            "body": "<xml><return_code>SUCCESS</return_code></xml>",
            "content_type": "text/xml; charset=UTF-8",
        }]

        empty = self.valid_fixture()
        empty["id"] = "http-post-empty-response"
        empty["operation"] = "miniapp.http.post-empty-response"
        empty["comparison"] = "http_error"
        empty["input"]["api_body"] = "{\"probe\":\"empty-post-response\"}"
        empty["input"]["token_responses"] = [empty["input"]["token_responses"][0]]
        empty["input"]["api_responses"] = [{
            "status": 200, "body": "", "content_type": "text/xml; charset=UTF-8"
        }]

        rows = self.api.load_http_fixtures(self.write_jsonl([xml, empty]))
        self.assertEqual([row["operation"] for row in rows], [
            "miniapp.http.post-xml-response", "miniapp.http.post-empty-response"
        ])

    def test_loads_dynamic_interceptor_fixture(self):
        path = Path(__file__).with_name("http") / "dynamic-interceptor-fixture.jsonl"
        rows = self.api.load_http_fixtures(path)
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["operation"], "miniapp.http.get-dynamic-interceptor")
        self.assertEqual(rows[0]["input"]["dynamic_headers"]["token_value"], "token-hook-v1")

    def test_loads_media_upload_and_common_upload_fixtures(self):
        folder = Path(__file__).with_name("http")
        media = self.api.load_http_fixtures(folder / "media-upload-fixture.jsonl")
        common = self.api.load_http_fixtures(folder / "common-upload-fixture.jsonl")
        self.assertEqual(media[0]["operation"], "miniapp.http.media-upload")
        self.assertEqual(common[0]["operation"], "miniapp.http.common-upload")
        self.assertEqual(common[0]["input"]["form_fields"]["description"], "phase1 multipart probe")

    def test_loads_media_download_fixture(self):
        path = Path(__file__).with_name("http") / "media-download-fixture.jsonl"
        rows = self.api.load_http_fixtures(path)
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["operation"], "miniapp.http.media-download")
        self.assertEqual(rows[0]["input"]["media_id"], "MEDIA-DOWNLOAD-001")

    def test_rejects_unsafe_upload_filename(self):
        fixture = self.valid_fixture()
        fixture["operation"] = "miniapp.http.media-upload"
        fixture["input"].pop("api_body")
        fixture["input"].update({
            "media_type": "image",
            "file_name": "../outside.bin",
            "file_bytes_hex": "00",
            "form_fields": {},
        })
        with self.assertRaisesRegex(ValueError, "safe basename"):
            self.api.load_http_fixtures(self.write_jsonl([fixture]))

    def test_rejects_different_default_headers_across_fixtures(self):
        first = self.valid_fixture()
        second = self.valid_fixture()
        second["id"] = "second"
        second["input"]["default_headers"]["X-WxRust-Probe"] = "different"
        with self.assertRaisesRegex(ValueError, "same default_headers"):
            self.api.load_http_fixtures(self.write_jsonl([first, second]))

    def test_file_download_semantic_and_error_fixtures(self):
        folder = Path(__file__).with_name("http")
        success = self.api.load_http_fixtures(
            folder / "media-download-file-fixture.jsonl"
        )
        missing = self.api.load_http_fixtures(
            folder / "media-download-file-no-disposition-fixture.jsonl"
        )
        rejected = self.api.load_http_fixtures(
            folder / "media-download-file-http-503-fixture.jsonl"
        )
        self.assertEqual(success[0]["comparison"], "file_semantics")
        self.assertEqual(success[0]["input"]["expected_file_extension"], ".bin")
        self.assertEqual(missing[0]["comparison"], "http_error")
        self.assertEqual(rejected[0]["comparison"], "http_error")

    def test_file_semantics_requires_explicit_expected_file_evidence(self):
        fixture = self.valid_fixture()
        fixture["operation"] = "miniapp.http.media-download-file"
        fixture["comparison"] = "file_semantics"
        fixture["input"].pop("api_body")
        fixture["input"]["media_id"] = "M1"
        with self.assertRaisesRegex(ValueError, "expected_file_prefix"):
            self.api.load_http_fixtures(self.write_jsonl([fixture]))

    def test_rejects_duplicate_scenario_ids(self):
        fixture = self.valid_fixture()
        with self.assertRaisesRegex(ValueError, "duplicate"):
            self.api.load_http_fixtures(self.write_jsonl([fixture, fixture]))

    def test_rejects_unknown_operation(self):
        fixture = self.valid_fixture()
        fixture["operation"] = "miniapp.http.fake"
        with self.assertRaisesRegex(ValueError, "operation"):
            self.api.load_http_fixtures(self.write_jsonl([fixture]))

    def test_rejects_missing_token_response(self):
        fixture = self.valid_fixture()
        fixture["input"]["token_responses"] = []
        with self.assertRaisesRegex(ValueError, "token_responses"):
            self.api.load_http_fixtures(self.write_jsonl([fixture]))

    def test_rejects_empty_fixture_file(self):
        with self.assertRaisesRegex(ValueError, "empty"):
            self.api.load_http_fixtures(self.write_jsonl([]))


if __name__ == "__main__":
    unittest.main()
