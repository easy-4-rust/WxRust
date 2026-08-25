# aispeech 孤立方法接线复核（Task 13）

日期：2026-08-25
计划：`docs/superpowers/plans/2026-08-24-wxrust-concurrency-native-architecture.md` Task 13
范围：`crates/wx-rust-aispeech` 全部 trait 方法（图谱分析提示 ~50 个 degree=1 方法；以源码为权威逐项复核）
Java 对照：`/Users/wandl/workspaces/workspace-github/WxJava/weixin-java-aispeech/src/main/java`

## 结论摘要

- **接口面 1:1 对齐 Java**：dialog 6/6、knowledge 17/17、service 门面 4/4 访问器，无 Java 方法在 Rust 缺失。
- **接线无缺口**：全部 23 个子服务方法 + 11 个门面方法（5 抽象 + 6 执行引擎默认实现）均有实现且经
  `WxAispeechServiceImpl::new_arc` 装配（dialog/knowledge 子服务在构造时以 `Weak<dyn WxAispeechService>`
  注入，门面 `dialog_service()`/`knowledge_service()` 可达）。**未发现需补接线的孤立方法**（图谱
  degree=1 提示为陈旧/粒度噪声：其计数含 bean 序列化字段等非方法实体）。
- **测试缺口为真**：dialog 6 方法此前 0 个 HTTP 级测试（Java 侧同样无
  `WxAispeechDialogServiceImplTest`，属双侧空白）；knowledge 17 方法中 9 个已测、8 个未测。
  本次补测后 **23/23 全覆盖**。

## 一、三向核对矩阵（method × impl / facade / test）

图例：✔=已接线且已验证；「测试」列括注测试函数（文件见 `crates/wx-rust-aispeech/tests/`）。

### 1.1 WxAispeechDialogService（6 方法）

| # | Rust 方法 | 对应 Java | impl | facade 路径 | 测试（本次后） |
|---:|---|---|:-:|:-:|---|
| 1 | `get_access_token` | `getAccessToken` | ✔ | `service.dialog_service()` | ✔ `test_get_access_token_writes_open_ai_token`（含 `setOpenAiToken` 副作用 + `X-APPID` + 签名复算） |
| 2 | `import_bot_json` | `importBotJson` | ✔ | 同上 | ✔ `test_import_publish_and_query_async_task`（mode/data 请求体 + `X-OPENAI-TOKEN`） |
| 3 | `publish_bot` | `publishBot` | ✔ | 同上 | ✔ 同上（返回 `request_id` + 字面量 `"{}"` 请求体） |
| 4 | `get_publish_progress` | `getPublishProgress` | ✔ | 同上 | ✔ `test_get_publish_progress` |
| 5 | `query_async_task` | `queryAsyncTask` | ✔ | 同上 | ✔ 同 2（state/progress/total_count 解析） |
| 6 | `query` | `query` | ✔ | 同上 | ✔ `test_query_encrypted_roundtrip`（请求 AES 加密 + 密文响应解密分支）、`test_query_plain_json_response`（明文响应分支 + `raw_answer`） |

执行引擎 `execute_dialog_post` 报错分支（`X-OPENAI-TOKEN`/`X-APPID` 缺失）由
`test_dialog_error_branches` 验证（断言报错且 0 次网络请求）。

### 1.2 WxAispeechKnowledgeService（17 方法）

| # | Rust 方法 | 对应 Java | impl | facade 路径 | 测试（本次后） |
|---:|---|---|:-:|:-:|---|
| 1 | `create_knowledge_by_file` | `createKnowledgeByFile` | ✔ | `service.knowledge_service()` | ✔（已有）`test_create_knowledge_by_file` |
| 2 | `create_knowledge_by_url` | `createKnowledgeByUrl` | ✔ | 同上 | ✔（新）`test_create_url_manual_list_update_delete_search_and_raw` |
| 3 | `create_knowledge_by_manual` | `createKnowledgeByManual` | ✔ | 同上 | ✔（新）同上 |
| 4 | `list_knowledge` | `listKnowledge` | ✔ | 同上 | ✔（新）同上（page/page_size 查询参数） |
| 5 | `list_knowledge_by_ids` | `listKnowledgeByIds` | ✔ | 同上 | ✔（已有）`test_list_knowledge_by_ids_and_move_progress` |
| 6 | `get_knowledge` | `getKnowledge` | ✔ | 同上 | ✔（已有）`test_knowledge_headers_and_missing_config` |
| 7 | `update_knowledge` | `updateKnowledge` | ✔ | 同上 | ✔（新）同 2（PUT + `enable_status` 请求体） |
| 8 | `update_manual_knowledge` | `updateManualKnowledge` | ✔ | 同上 | ✔（已有）`test_update_manual_move_and_tag_apis` |
| 9 | `delete_knowledge` | `deleteKnowledge` | ✔ | 同上 | ✔（新）同 2（DELETE + 恒 true） |
| 10 | `update_knowledge_tags` | `updateKnowledgeTags` | ✔ | 同上 | ✔（已有）`test_update_manual_move_and_tag_apis`（含空列表/空 tagId 短路负例） |
| 11 | `search_knowledge` | `searchKnowledge` | ✔ | 同上 | ✔（新）同 2（keyword/knowledge_base_id 查询参数） |
| 12 | `move_knowledge` | `moveKnowledge` | ✔ | 同上 | ✔（已有）`test_update_manual_move_and_tag_apis` |
| 13 | `get_move_progress` | `getMoveProgress` | ✔ | 同上 | ✔（已有）`test_list_knowledge_by_ids_and_move_progress` |
| 14 | `create_knowledge_base_tag` | `createKnowledgeBaseTag` | ✔ | 同上 | ✔（已有）`test_update_manual_move_and_tag_apis` |
| 15 | `update_knowledge_base_tag` | `updateKnowledgeBaseTag` | ✔ | 同上 | ✔（已有）同上 |
| 16 | `post_raw` | `postRaw` | ✔ | 同上 | ✔（新）同 2（请求体原样透传） |
| 17 | `get_raw` | `getRaw` | ✔ | 同上 | ✔（新）同 2（空值查询参数跳过） |

### 1.3 WxAispeechService 门面（4 访问器 + 7 执行引擎）

| # | Rust 方法 | 对应 Java | impl | facade | 测试 |
|---:|---|---|:-:|:-:|---|
| 1 | `config_storage` | `getConfigStorage` | ✔（impl 覆写） | 即门面自身 | ✔（已有）`test_service_impl_construction` / `test_service_set_config_storage` |
| 2 | `set_config_storage` | `setConfigStorage`（含 initHttp 重建客户端） | ✔ | 同上 | ✔（已有）`test_service_set_config_storage` |
| 3 | `dialog_service` | `getDialogService` | ✔（trait 默认 None，impl 覆写装配） | 即门面自身 | ✔（已有）`test_service_impl_construction`；本次全部 dialog 测试均经此路径调用 |
| 4 | `knowledge_service` | `getKnowledgeService` | ✔ | 同上 | ✔ 同上 |
| 5 | `http_client` | ADAPTED（Java 无：reqwest 承载 `HttpComponentsClientBuilder`） | ✔ | 同上 | ✔（已有）`test_service_impl_construction` |
| 6-11 | `execute_dialog_post` / `execute_knowledge_get/post/put/delete/multipart_post` | Java `WxAispeechServiceImpl` protected 方法 | ✔（trait 默认实现） | 子服务经 `Weak` 升级调用 | ✔ dialog 引擎经 §1.1 全部测试；knowledge 引擎经 §1.2 全部测试 |

## 二、Java 对齐（parity）表

Java 接口方法 vs Rust trait 方法逐一比对（源文件：
`api/WxAispeechDialogService.java`、`api/WxAispeechKnowledgeService.java`、`api/WxAispeechService.java`）：

| Java 接口 | Java 方法数 | Rust 对应 | Rust 方法数 | 缺口 |
|---|---:|---|---:|---|
| `WxAispeechDialogService` | 6 | `WxAispeechDialogService` | 6 | 无（1:1，语义逐项对照 impl 亦一致） |
| `WxAispeechKnowledgeService` | 17 | `WxAispeechKnowledgeService` | 17 | 无（`File` 参数 ADAPTED 为字节+文件名；`Object` 请求体 ADAPTED 为 JSON 字符串） |
| `WxAispeechService`（接口） | 4 | `WxAispeechService` | 4 访问器 + `http_client` + 6 执行引擎默认方法 | 无缺口；Rust 增量为 ADAPTED：引擎方法以 trait 默认实现表达 Java 具体类 protected 方法，`http_client` 承载 `initHttp` |

已知 ADAPTED 项（非缺口，均有注释说明）：
- `createKnowledgeByFile` 的 `java.io.File` → `file_name: &str, file_bytes: &[u8]`；
- `postRaw` 的 `Object requestBody` → `Option<&str>`（调用方序列化，`None` = Java `toBody(null)` 的 `"{}"`）；
- `getAccessToken`/`query` 等 `String` 可空参数 → `Option<&str>`；
- `HttpComponentsClientBuilder` 代理账号密码为 `PLATFORM_NA`（reqwest 不内建），代理 host/port 已支持。

## 三、处置清单（disposition）

| 项 | 处置 | 理由 |
|---|---|---|
| dialog 6 方法 impl | 已接线（无需改动） | `WxAispeechDialogServiceImpl` 全量实现，路径/请求体/解析逐行对照 Java |
| knowledge 17 方法 impl | 已接线（无需改动） | `WxAispeechKnowledgeServiceImpl` 全量实现 |
| 门面装配 | 已接线（无需改动） | `new_arc` 构造时注入两个子服务，getter 可达；`test_service_impl_construction` 断言 `is_some` |
| dialog 6 方法测试 | **补测**（新增 `tests/wx_aispeech_dialog_service_test.rs`，6 个测试） | 双侧空白：Java 无 dialog impl 测试，Rust 原亦无；属真缺口 |
| knowledge 8 方法测试 | **补测**（`tests/wx_aispeech_knowledge_service_test.rs` 追加 1 个测试覆盖 8 方法） | url/manual 创建、list、update、delete、search、post_raw/get_raw 原无任何测试 |
| knowledge 9 方法测试 | 保留既有 | 已有 3 个镜像 Java 的测试 + 1 个签名/缺配置测试覆盖 |
| Java parity | 无需改动 | 接口面 1:1（见第二节） |
| 图谱 ~50 degree=1 提示 | 判定为噪声 | 其清单含 bean 字段访问器/serde 结构等非服务方法实体；源码逐项复核后未发现孤立（未接线）方法 |
| knowledge 测试占位 aesKey（50 位 base64 → 37 字节，非法 AES 长度） | 保留不动 | 该值仅在 knowledge 测试中作配置占位、从不进入 AES 路径；Java 同值同样无法通过 `SecretKeySpec`。dialog 测试改用合法 43 位密钥（32 字节）并在注释说明 | 

## 四、前后对照

| 指标 | 复核前 | 复核后 |
|---|---:|---:|
| wx-rust-aispeech 测试数 | 40 | **47**（+6 dialog，+1 knowledge） |
| dialog 方法 HTTP 级测试覆盖 | 0/6 | **6/6** |
| knowledge 方法测试覆盖 | 9/17 | **17/17** |
| 子服务 trait 方法测试覆盖合计 | 9/23 | **23/23** |
| 接线缺口 | 0 | 0（复核确认为零） |
| Java 接口 parity 缺口 | 0 | 0（复核确认为零） |

新增测试文件：`crates/wx-rust-aispeech/tests/wx_aispeech_dialog_service_test.rs`（新文件，
含请求头记录能力的 MockServer 变体，照抄 knowledge 测试的 mock 模式）。

回归门禁：`cargo test -p wx-rust-aispeech` 47 全绿；`cargo test --workspace` 全绿；
`cargo clippy --workspace --all-targets -- -D warnings` 干净；`cargo fmt --all` 无 diff。
