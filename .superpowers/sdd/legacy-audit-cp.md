# wx-rust-cp 存量语义审计报告

**日期**: 2026-08-27
**基线测试数**: 521 passed, 0 failed
**审计后测试数**: 522 passed, 0 failed (+1 新增)
**Clippy**: clean (-D warnings)
**Fmt**: clean

## 审计范围

对 `crates/wx-rust-cp/src/api/impl/` 下全部服务实现文件，逐方法对照 Java
`WxJava/weixin-java-cp/src/main/java/me/chanjar/weixin/cp/api/impl/` 进行三向语义审计：

1. URL 常量 == Java `WxCpApiPathConsts`
2. 参数字段名 == Java `@SerializedName` / Gson 字段名
3. 响应反序列化目标 == Java Result bean
4. 方法语义（GET/POST、token 拼接、特殊逻辑）

## 审计结果汇总

| Service | 方法数 | 缺陷数 | 状态 |
|---------|--------|--------|------|
| wx_cp_user_service_impl | 15 | 1 | FIXED |
| wx_cp_message_service_impl | 5 | 0 | OK |
| wx_cp_external_contact_service_impl | ~40 | 0 | OK |
| wx_cp_media_service_impl | 10 | 0 | OK |
| wx_cp_oauth2_service_impl | 8 | 0 | OK |
| wx_cp_tag_service_impl | 7 | 0 | OK |
| wx_cp_agent_service_impl | 4 | 0 | OK |
| wx_cp_menu_service_impl | 6 | 0 | OK |
| wx_cp_department_service_impl | 6 | 0 | OK |
| wx_cp_oa_service_impl | 18 | 0 | OK |
| **合计** | **~119** | **1** | |

## 缺陷详情

### Defect #1: `authenticate` URL 拼接错误

- **文件**: `wx_cp_user_service_impl.rs` L37-45
- **严重度**: 高（生产请求 URL 错误，userid 参数值丢失）
- **Java 行为**: `get(config.getApiUrl(USER_AUTHENTICATE + userId), null)`
  URL = `https://qyapi.weixin.qq.com/cgi-bin/user/authsucc?userid=testUser`
- **Rust 缺陷行为**: `get(&config.api_url(USER_AUTHENTICATE), user_id)`
  管线追加 query 后 URL = `.../user/authsucc?userid=&testUser`
  （USER_AUTHENTICATE 已含 `?userid=`，user_id 被当作独立 query 参数用 `&` 追加）
- **修复**: `format!("{}{user_id}", config.api_url(USER_AUTHENTICATE))`，与 Java 一致将 userId 直接拼接在 URL 末尾
- **测试**: `test_authenticate_url_concatenation` — 断言 URL 包含 `userid=testUser123` 且不包含 `userid=&`

## 逐 Service 审计记录

### 1. wx_cp_user_service_impl (15 方法)

| 方法 | URL | 参数 | 响应 | 语义 | 结果 |
|------|-----|------|------|------|------|
| authenticate | ~~BUG~~ FIXED | - | - | GET, userId 拼 URL | FIXED |
| create | OK USER_CREATE | user.toJson() | - | POST | OK |
| update | OK USER_UPDATE | user.toJson() | - | POST | OK |
| delete | OK USER_DELETE/USER_BATCH_DELETE | 单:GET, 多:POST useridlist | - | 分支逻辑 | OK |
| get_by_id | OK USER_GET+userid | - | WxCpUser.fromJson | GET | OK |
| list_by_department | OK USER_LIST+departId | fetch_child/status params | userlist array | GET | OK |
| list_simple_by_department | OK USER_SIMPLE_LIST+departId | 同上 | userlist array | GET | OK |
| invite | OK BATCH_INVITE | user/party/tag arrays | WxCpInviteResult | POST | OK |
| user_id2_openid | OK USER_CONVERT_TO_OPENID | userid+agentid? | openid/appid map | POST | OK |
| openid2_user_id | OK USER_CONVERT_TO_USERID | openid | userid string | POST | OK |
| get_user_id | OK GET_USER_ID | mobile | userid string | POST | OK |
| get_user_id_by_email | OK GET_USER_ID_BY_EMAIL | email+email_type | userid string | POST | OK |
| get_external_contact | OK GET_EXTERNAL_CONTACT+userId | - | WxCpExternalContactInfo | GET | OK |
| get_join_qr_code | OK GET_JOIN_QR_CODE+sizeType | - | join_qrcode string | GET | OK |
| get_active_stat | OK GET_ACTIVE_STAT | date yyyy-MM-dd | active_cnt i32 | POST | OK |
| userid_to_open_userid | OK USERID_TO_OPEN_USERID | userid_list array | WxCpUseridToOpenUseridResult | POST | OK |
| open_userid_to_userid | OK OPEN_USERID_TO_USERID | open_userid_list+source_agentid | WxCpOpenUseridToUseridResult | POST | OK |
| get_user_list_id | OK USER_LIST_ID | cursor+limit? | WxCpDeptUserResult | POST | OK |

### 2. wx_cp_message_service_impl (5 方法)

| 方法 | URL | 参数 | 响应 | 语义 | 结果 |
|------|-----|------|------|------|------|
| send | OK MESSAGE_SEND | agentId 回填+message.toJson() | WxCpMessageSendResult | POST | OK |
| get_statistics | OK GET_STATISTICS | time_type | WxCpMessageSendStatistics | POST | OK |
| send_linked_corp_message | OK LINKEDCORP_MESSAGE_SEND | agentId 回填+toJson() | WxCpLinkedCorpMessageSendResult | POST | OK |
| send_school_contact_message | OK EXTERNAL_CONTACT_MESSAGE_SEND | agentId 回填+toJson() | WxCpSchoolContactMessageSendResult | POST | OK |
| recall | OK MESSAGE_RECALL | msgid | - | POST | OK |

### 3. wx_cp_external_contact_service_impl (~40 方法)

全部方法 URL/参数/响应/语义与 Java 一致。关键验证点：
- add_contact_way: POST 内层 contact_way（非外层 info），100 人限制
- get_contact_detail: external_userid 拼 URL，cursor 可选追加
- list_external_contacts: GET list?userid=，84061 返回空列表
- transfer_external_contact: handover_userid/takeover_userid 字段名
- list_group_chat: cursor/limit 默认值写入，owner_filter 条件组装
- upload_attachment: multipart 上传，media_type+attachment_type 拼 URL

### 4. wx_cp_media_service_impl (10 方法)

全部方法 URL/参数/响应/语义与 Java 一致。关键验证点：
- upload: MEDIA_UPLOAD+mediaType 拼接
- download: GET MEDIA_GET?access_token=&media_id=，JSON Content-Type 判断为错误
- upload_img: IMG_UPLOAD POST multipart，取 url 字段
- upload_by_url: UPLOAD_BY_URL POST，取 jobid
- upload_by_url_result: GET_UPLOAD_BY_URL_RESULT POST {"jobid":...}

### 5. wx_cp_oauth2_service_impl (8 方法)

全部方法 URL/参数/响应/语义与 Java 一致。关键验证点：
- build_authorization_url_with_scope: appid/redirect_uri/agentid(state 非空追加)
- get_user_info_with_agent_id: String.format(GET_USER_INFO, code, agentId) 用 replace 模拟
- get_school_user_info: String.format(GET_SCHOOL_USER_INFO, code)
- get_user_detail: POST GET_USER_DETAIL {"user_ticket":...}
- get_auth_user_info: GET GET_USER_AUTH_INFO?code=，字段 userid/openid/user_ticket/external_userid
- get_tfa_info: POST GET_TFA_INFO {"code":...}

### 6-9. tag/agent/menu/department (共 23 方法)

全部方法 URL/参数/响应/语义与 Java 一致。

### 10. wx_cp_oa_service_impl (18 方法)

全部方法 URL/参数/响应/语义与 Java 一致。关键验证点：
- get_checkin_data: opencheckindatatype/starttime/endtime/useridlist，1 月限制
- get_approval_info_with_new_cursor: new_cursor/size/filters，size 1-100 校验
- get_dial_record: offset/limit/start_time/end_time，30 天限制
- set_one_user_quota: userid/vacation_id/leftduration/time_attr/remarks

## 文件变更清单

- `crates/wx-rust-cp/src/api/impl/wx_cp_user_service_impl.rs`: 修复 `authenticate` URL 拼接 + 新增测试模块
