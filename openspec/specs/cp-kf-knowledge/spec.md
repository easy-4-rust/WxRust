# cp-kf-knowledge Specification

## Purpose
为企业微信客服提供完整的知识库分组和问答维护能力，让调用方通过现有客服服务管理问题、相似问法、多媒体答案与分页结果，保持企业微信的错误语义和字段格式。

## Requirements

### Requirement: 分组管理
SDK SHALL 提供知识分组新增、删除、修改、列表查询，通过 POST 调用 /cgi-bin/kf/knowledge/ 下的 add_group、del_group、mod_group、list_group。分组 SHALL 支持 group_id、name、is_default；新增结果返回 group_id。

#### Scenario: 分组四种操作
- **WHEN** 分别执行新增、删除、修改和列表操作
- **THEN** 请求到达对应路径，删除请求携带 group_id，新增和修改保留传入字段，列表解析 group_list。

### Requirement: 问答管理
SDK SHALL 提供问答新增、删除、修改、列表查询，通过相同前缀的 add_intent、del_intent、mod_intent、list_intent 路径。问答 SHALL 支持 group_id、intent_id、question、similar_questions 和 answers，新增结果返回 intent_id。

#### Scenario: 问答四种操作
- **WHEN** 分别执行四种问答操作
- **THEN** 请求体与路径对应，删除携带 intent_id，结果正确解析问答标识或 intent_list。

### Requirement: 分页与空值语义
分组列表 SHALL 接受 cursor、limit、group_id；问答列表 SHALL 额外接受 intent_id。缺省参数 MUST 省略，所有参数缺省时发送 {}。分页结果 SHALL 保留 next_cursor 和整数 has_more，不自动无界拉取所有页。

#### Scenario: 缺省与显式空串
- **WHEN** cursor 未提供或被显式设置为空串
- **THEN** 前者不输出 cursor，后者保留空串；其他参数同样区分缺省与已提供值。

#### Scenario: 多页结果
- **WHEN** 响应包含 has_more=1 和 next_cursor
- **THEN** 返回游标供调用方继续请求，下一次请求原样发送所提供游标。

### Requirement: 多媒体答案与嵌套问答
SDK SHALL 保留文本、图片、视频、链接和小程序附件的已定义字段，并兼容相似问法与答案位于问答顶层或 question 内部的上游响应形态，不丢失嵌套内容。

#### Scenario: 完整附件反序列化
- **WHEN** 问答包含文本内容、图片或视频 media_id/name、链接 title/pic_url/desc/url、小程序 title/thumb_media_id/appid/pagepath
- **THEN** 所有字段可被调用方读取并结构等价地重新序列化。

### Requirement: 复用客服认证与错误
知识库调用 SHALL 复用现有客服服务的认证和配置入口，企业微信 errcode 非零、网络失败或无效 JSON MUST 作为失败返回，不伪造成功结果。

#### Scenario: 业务错误
- **WHEN** 知识库接口返回非零 errcode
- **THEN** 调用方收到包含原错误码的错误，既有客服操作仍可使用。
