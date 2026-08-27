# Legacy Semantic Audit: wx-rust-qidian & wx-rust-aispeech

Date: 2026-08-27

## Scope

### qidian (4 impl files + 1 enum + 1 service trait)

| Rust file | Java counterpart | Methods audited |
|---|---|---|
| `api/impl/base_wx_qidian_service_impl.rs` | `BaseWxQidianServiceImpl.java` | `retry_loop`, `execute_with_retry`, `execute_get_via_pipeline`, `execute_post_via_pipeline`, `execute_via_pipeline`, `execute_internal` |
| `api/impl/wx_qidian_service_impl.rs` | `WxQidianServiceImpl.java` (via `BaseWxQidianServiceImpl`) | `config_storage`, `set_config_storage`, `add_config_storage`, `remove_config_storage`, `set_multi_config_storages`, `set_multi_config_storages_with_default`, `switchover`, `switchover_to`, `http_client`, `retry_sleep_millis`, `max_retry_times`, `set_retry_sleep_millis`, `set_max_retry_times`, `dial_service`, `call_data_service` |
| `api/impl/wx_qidian_dial_service_impl.rs` | `WxQidianDialServiceImpl.java` | `ivr_dial`, `get_ivr_list` |
| `api/impl/wx_qidian_call_data_service_impl.rs` | `WxQidianCallDataServiceImpl.java` | `get_switch_board_list` |
| `api/wx_qidian_service.rs` (trait defaults) | `BaseWxQidianServiceImpl.java` | `check_signature`, `get_access_token[_with_force]`, `do_get_access_token_request`, `get_ticket[_with_force]`, `get_jsapi_ticket[_with_force]`, `create_jsapi_signature`, `short_url`, `build_qr_connect_url`, `get_callback_ip`, `net_check`, `clear_quota`, `get`, `get_by_url`, `post`, `post_by_url`, `post_json`, `post_json_by_url`, `extract_access_token`, `format_url` |
| `enums/wx_qidian_api_url.rs` | `WxQidianApiUrl.java` | All URL constants (OAuth2, Other, Dial, CallData) verified character-by-character |

### aispeech (4 impl files + 1 sign util + 1 service trait + 10 bean files)

| Rust file | Java counterpart | Methods audited |
|---|---|---|
| `api/impl/wx_aispeech_service_impl.rs` | `WxAispeechServiceImpl.java` | `new_arc`, `build_http_client`, `config_storage`, `set_config_storage`, `http_client`, `dialog_service`, `knowledge_service` |
| `api/wx_aispeech_service.rs` (trait defaults) | `WxAispeechServiceImpl.java` | `execute_dialog_post`, `execute_knowledge_get`, `execute_knowledge_post`, `execute_knowledge_put`, `execute_knowledge_delete`, `execute_knowledge_multipart_post`, `to_body`, `build_knowledge_headers`, `execute_request`, `now_seconds`, `random_nonce` |
| `api/impl/wx_aispeech_dialog_service_impl.rs` | `WxAispeechDialogServiceImpl.java` | `get_access_token`, `import_bot_json`, `publish_bot`, `get_publish_progress`, `query_async_task`, `query`, `ensure_success`, `looks_like_json` |
| `api/impl/wx_aispeech_knowledge_service_impl.rs` | `WxAispeechKnowledgeServiceImpl.java` | `create_knowledge_by_file`, `create_knowledge_by_url`, `create_knowledge_by_manual`, `list_knowledge`, `list_knowledge_by_ids`, `get_knowledge`, `update_knowledge`, `update_manual_knowledge`, `delete_knowledge`, `update_knowledge_tags`, `search_knowledge`, `move_knowledge`, `get_move_progress`, `create_knowledge_base_tag`, `update_knowledge_base_tag`, `post_raw`, `get_raw`, `parse_knowledge_info_list` |
| `util/wx_aispeech_sign_util.rs` | `WxAispeechSignUtil.java` | `calc_dialog_sign` (MD5 chain), `calc_knowledge_signature` (HmacSHA256), `encrypt_aes_cbc_to_base64`, `decrypt_aes_cbc_from_base64`, `decode_aes_key` |
| Bean files (10) | Java counterparts | `AispeechApiResponse`, `AsyncTaskResult`, `BotIntent`, `DialogQueryRequest`, `DialogResult`, `PublishProgress`, `KnowledgeInfo`, `KnowledgeListResult`, `KnowledgeManualCreateRequest`, `KnowledgeMoveProgress`, `KnowledgeMoveRequest`, `KnowledgeTagRequest`, `KnowledgeUpdateRequest`, `KnowledgeUrlCreateRequest` |

## Three-way verification results

### URLs -- PASS

All URL constants in `wx_qidian_api_url.rs` match Java `WxQidianApiUrl` enum values character-by-character (prefix + path, including host config substitution via `WxQidianHostConfig::build_url`).

Aispeech URLs are constructed inline from `config.dialog_api_base_url()` / `config.knowledge_api_base_url()` + path strings; all path strings match Java counterparts.

### Serialized field names vs @SerializedName -- 1 DEFECT FOUND AND FIXED

**Defect: `AsyncTaskResult` -- 5 fields used snake_case JSON keys instead of camelCase**

Java `AsyncTaskResult` has these fields with NO `@SerializedName` annotation, meaning Gson serializes them as their Java field names (camelCase):

| Java field | Gson JSON key | Rust was (wrong) | Rust fixed to |
|---|---|---|---|
| `totalCount` | `totalCount` | `total_count` | `totalCount` |
| `successCount` | `successCount` | `success_count` | `successCount` |
| `failCount` | `failCount` | `fail_count` | `failCount` |
| `successSkillInfo` | `successSkillInfo` | `success_skill_info` | `successSkillInfo` |
| `successSkillInfoList` | `successSkillInfoList` | `success_skill_info_list` | `successSkillInfoList` |

Files changed:
- `crates/wx-rust-aispeech/src/bean/dialog/async_task_result.rs` -- serde rename attributes
- `crates/wx-rust-aispeech/tests/aispeech_comprehensive_test.rs` -- test JSON fixtures
- `crates/wx-rust-aispeech/tests/wx_aispeech_dialog_service_test.rs` -- mock response JSON

All other bean files verified correct:
- `DialogQueryRequest`: all `@SerializedName` fields correctly mapped
- `DialogResult`: all `@SerializedName` fields correctly mapped
- `PublishProgress`: `end_time` correctly renamed
- `KnowledgeInfo`: all fields correctly renamed
- `KnowledgeMoveRequest`: all fields correctly renamed
- `KnowledgeTagRequest`: `sort_order` correctly renamed
- `KnowledgeUpdateRequest`: `enable_status` correctly renamed
- `KnowledgeListResult`: `page_size` correctly renamed
- `KnowledgeMoveProgress`: `task_id` correctly renamed

### Response field parsing -- PASS

All response parsing matches Java: `AispeechApiResponse<T>` structure (code/msg/request_id/data), `ensureSuccess` logic (code null or != 0 throws), `looksLikeJson` heuristic (starts with `{` or `[`).

### Signature/crypto logic -- PASS

- `calc_dialog_sign`: MD5(token + timestamp + nonce + md5(body)) -- verified equal to Java `DigestUtils.md5Hex(defaultString(token) + timestamp + defaultString(nonce) + md5Hex(body))`
- `calc_knowledge_signature`: HmacSHA256(secretKey, timestamp + "\n" + nonce + "\n" + requestId + "\n" + body) -- verified equal
- AES-CBC encrypt/decrypt: key=base64(aesKey+"="), IV=key[:16], PKCS7 padding -- verified equal
- `decode_aes_key`: base64 lenient decode (append "=", trim padding, handle incomplete quantum) -- verified equal to Java `Base64.decodeBase64(defaultString(aesKey) + "="")`

## Test results

```
wx-rust-qidian:   45 tests, 0 failed
wx-rust-aispeech: 47 tests, 0 failed
clippy: clean
fmt:    clean
```

## Summary

Audited ~50 methods across 2 crates against Java source. Found and fixed 1 real defect: `AsyncTaskResult` had 5 fields with incorrect JSON serialization keys (snake_case instead of camelCase), which would cause deserialization failures when parsing real API responses.
