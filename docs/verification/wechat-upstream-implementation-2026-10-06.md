# 微信上游四项能力实施与验证报告

日期：2026-10-06（Asia/Shanghai）。仓库基线为 main / 260d800，workspace 0.1.4。四项均已实现，OpenSpec 为唯一规格事实源；尚未执行真实微信账号联调。全部离线验收门禁已闭合，43/43 项任务完成，四份主规格已同步，变更已归档。

## 交付与边界

| 优先级 | 内容 | 本次实现 |
|---|---|---|
| P1 | 个人主体虚拟支付 | 四个独立模型、本地前端参数生成、双 HMAC、JSON/XML 商品发货通知 |
| P1 | 企业微信客服知识库 | 六个顶层模型及附件、分组/问答八个端点、分页、缺省与显式空串、认证及错误复用 |
| P1 | 旅客运输电子发票 | 全行业字段、i64 数量及负折扣、现有 V3 签名、密文透传、202 空响应受理 |
| P2 | 独立微信小店 | wx-rust-store、26 个经营子服务、独立认证、经营模型与方法覆盖、上传、回调、兼容迁移及发布清单 |

方案与接入说明见 [接入及迁移指南](../wechat-upstream-capabilities.md)。正式变更见 [proposal](../../openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities/proposal.md)、[design](../../openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities/design.md)、[tasks](../../openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities/tasks.md)。

个人支付入口计算并返回 mode/signData/paySig/signature，不生成网络订单、不返回 appKey/sessionKey。知识库与旅客发票采用现有客户端的扩展 trait 与 blanket impl，保留旧自定义客服/发票 trait 的必需方法。旅客发票当前验证公钥模式；调用方须预加密并配置匹配的 public_key_id，自动平台证书模式仍受现有支付 SDK 能力限制。202 表示受理，最终开票结果须查询或接收通知。

Store 与 Channel 是独立类型及客户端。Store 不依赖 Channel crate；旧 Channel 源码和导入保留。Store 经营模型使用 Option，None 省略，Some(空串或零) 保留；迁移时由业务方逐字段决定旧默认值如何转换。协议错误码/错误信息通过 Rust 基础响应适配保留既有默认值。

## 固定上游来源

本次覆盖固定提交，不将其表述为截至 10 月 6 日微信所有 API 的最新全集。

| 目标 | WxJava 固定提交 |
|---|---|
| U1 | [017421583aca8017aff5c5a1ba79fe58fd006bc2](https://github.com/binarywang/WxJava/commit/017421583aca8017aff5c5a1ba79fe58fd006bc2) |
| U2 | [c1591bae9c041ab82914d9129c10ee8cb08f9997](https://github.com/binarywang/WxJava/commit/c1591bae9c041ab82914d9129c10ee8cb08f9997) |
| U3 | [474f4c7a0fc549032efad3c029fa9a981bd82f9a](https://github.com/binarywang/WxJava/commit/474f4c7a0fc549032efad3c029fa9a981bd82f9a) |
| U4 | [230ed0a696855dc50a954331d8c0a023f15b3b0b](https://github.com/binarywang/WxJava/commit/230ed0a696855dc50a954331d8c0a023f15b3b0b) |

Store 完整树保存为 [store-upstream-tree.json](../../openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities/store-upstream-tree.json)，truncated=false。643 个 main Java 对象与 297 条公开方法/重载见 [store-coverage.json](../../openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities/store-coverage.json)，含源码 SHA-256、实现路径和测试引用。清单核验不存在无映射方法、无测试引用方法或无效 Rust 路径；这些静态结果不替代实际行为测试。

Java HTTP 后端由 reqwest 实现；Jackson/JSON/XML 工具通过 serde 与实际解析路径适配；Java Redis/Redisson 配置不声明为已提供的 Store 后端。上游 closeOrder 本身返回内部错误 -99，不发送请求；Store 保留并测试这个行为，不能用于实际关单。专属直播、Finder、联盟、留资及达人罗盘服务保留在 Channel。

## 本次实际验证

所有计数均取本次运行输出，不使用历史报告的测试数量。日志摘要及 SHA-256 见 [verification-evidence.json](wechat-upstream-verification-evidence-2026-10-06.json)。本地完整日志保存在忽略目录 .codegraph，摘要保存到正式文档。

| 验证 | 实际命令或范围 | 结果 |
|---|---|---|
| Store 完整测试 | cargo test -p wx-rust-store --all-features --offline | 1,392 通过，0 失败 |
| 工作区全功能回归 | cargo test --workspace --all-features --offline（REDIS_URL 本地真实 Redis） | 5,021 通过，0 失败，1 ignored，0 filtered |
| Redis 纯类型补测 | common 的 batch_d_common_beans / rust_obligation_value_add 中 redis_ 测试 | 2 项类型补测通过；随后 14 项真实 Redis 集成测试及未过滤工作区回归均通过 |
| 工作区检查 | cargo check --workspace --all-features --offline | 通过 |
| 固定 CI 工具链检查 | cargo +1.97.1 clippy --workspace --all-targets --all-features --offline -- -D warnings | 通过 |
| 最低 Rust 版本 | cargo +1.89.0 check --workspace --all-features --offline | 通过，退出码 0 |
| 格式 | cargo fmt --all -- --check | 通过 |
| 四项示例 | cargo check -p wx-rust-miniapp -p wx-rust-cp -p wx-rust-pay -p wx-rust-store --examples --offline | 通过 |
| 独立 consumer | cargo run --manifest-path tests/fixtures/store_consumer/Cargo.toml --offline | 本地认证与真实经营入口、响应内容断言通过 |
| 双模块 consumer | cargo run --manifest-path tests/fixtures/channel_store_consumer/Cargo.toml --offline | 旧导入、服务装配、显式类型转换、并发等价 GET 请求/响应及独立 token 通过 |
| 并发正确性 | cargo +1.97.1 bench -p ... --bench ... --offline -- --test | CI 的 release 单次断言模式通过：两个 common 场景、miniapp 1000 并发单飞；不表述为性能指标达标 |
| 打包 | cargo package -p wx-rust-store --allow-dirty --no-verify --offline | 747 文件，2.3 MiB / 压缩 276.6 KiB；未执行 registry 包验证构建或发布 |
| 源码门禁 | 生产 wildcard、todo!/unimplemented!、空函数、mod.rs 类型、block_on 等价扫描 | 无违规；测试模块与上游明确不支持行为单独识别 |
| OpenSpec | 活动变更 strict；归档后 validate --all / --archived --strict --no-interactive | 活动变更通过；4 份主规格、1 个归档检查均通过，43/43 项完成；17 条需求与原增量内容一致 |

Store 的 1,392 项已包含 433 个固定 Java 字段 fixture、425 个可空/缺省场景、267 个经营接口与上传测试，其他用例覆盖回调、分页、装配、错误及业务模型。246 条生成契约检查路径与错误传播；其余业务测试检查请求字段和成功响应。不得把全部生成契约表述为全部线上成功交易验证。

### 规格场景与证据

| 规格场景 | 测试或验证入口 |
|---|---|
| U1 正常生成、attach 缺省、固定双签名 | xpay_virtual_payment_test::virtual_payment_matches_upstream_utf8_signatures |
| U1 JSON/XML 等价、旧消息回归 | xpay_virtual_payment_test::goods_notification_json_and_xml_preserve_nested_fields；原 callback / xpay_subscribe_download_test |
| U2 分组与问答四种操作、可选字段、两页游标 | wx_cp_kf_service_impl::tests::knowledge_eight_endpoints_and_optional_filters |
| U2 完整附件、顶层与嵌套问答 | kf_knowledge_test::knowledge_preserves_both_question_shapes_and_all_attachments |
| U2 业务失败、坏 JSON、网络失败 | knowledge_propagates_bad_json_and_business_errors；knowledge_propagates_connection_failure；现有 cp 错误及客服回归 |
| U3 成功受理、密文、平台标识、请求签名头 | partner_invoice_test::passenger_invoice_accepts_202_empty_body_and_preserves_ciphertexts；原 V3 签名验证测试 |
| U3 全字段、大数量、负折扣、可选旅客信息 | partner_invoice_test::passenger_invoice_preserves_fields_large_integer_and_negative_discount |
| U3 失败、旧普通开票/查询/冲红 | passenger_invoice_propagates_payment_failure；partner_invoice_test 原 24 项及支付 V3 错误回归 |
| U4 独立接入、真实认证及响应 | store_consumer 外部工程；Store 依赖树无 Channel |
| U4 全量清单、字段与空值 | store-coverage.json；upstream_wire_fixtures；upstream_nullable_models |
| U4 装配、认证隔离、主要经营链路及上传 | service_assembly；sub_domain_store_shop；semantic / batch 服务测试 |
| U4 视频号边界、旧应用及分阶段迁移 | Store 公共导出审查；Channel 回归；channel_store_consumer；迁移指南 |
| U4 发布准备 | 工作区 manifest、release workflow、publish 顺序、747 文件包检查及独立 consumer |

### 失败驱动与修复

初始新增 U1 模型/回调字段、U2 方法、U3 模型及 202 行为缺失导致测试或编译失败，随后实施并回归。Store 的商品赠品入口缺失编译失败保存在 store-gift-red.log；可空字段测试在修复前因 null 无法解析为 i64 失败，保存在 store-nullable-red.log。后续修复了完整上游字段差异、stock flow 包装解析、图片下载主机配置、经营模型可空语义与服务参数构造。425 个空值场景与 433 个完整字段场景同时通过。

## 验收过程与剩余风险

1. 完整工作区 all-features 首次执行因 14 项 Redis 集成测试启动服务失败，退出 101：默认路径 /opt/homebrew/bin/redis-server 在 Windows 不存在，且测试使用 Unix socket。没有把这些测试改成忽略或删除；最终回归排除 redis_，另补测其中 2 项纯类型测试。后续增加 REDIS_URL 外部真实 Redis 接入，14 项原测试全部通过，最终工作区不排除任何测试。CI Redis 步骤显式设置 /usr/bin/redis-server，保留原 Unix socket 启动路径。
2. 用户授权安装 Rust 1.89.0 最小工具链后，cargo +1.89.0 check --workspace --all-features --offline 实际通过，退出码 0。原默认 stable 工具链未切换。
3. 并发正确性已补齐固定工具链 1.97.1 下的 release 单次断言模式；未宣称延迟或吞吐性能指标达标。
4. E 盘在中途全功能重跑及 consumer 构建时耗尽空间，相关尝试退出 101。自动审批拒绝递归缓存清理后，将最终构建产物改存 C 盘临时目录，关闭增量缓存与 debug 符号后完成重跑；未通过删除源码解决问题。
5. 未执行真实支付、发票、退款、提现或微信账号联调，未发布包。接入方需验证权限、平台密钥标识、业务状态与通知安全。

OpenSpec planning artifacts 已齐全；CLI status 的 isComplete 仅表示规划产物完整，不是实施或环境门禁通过证明。完整性检查有真实实现和行为证据，正确性检查包括签名、密文、空值、账号隔离及错误路径，一致性检查确认未修改旧 Channel 契约且没有第二套规格。Redis、release 并发与 MSRV 门禁均已闭合。OpenSpec CLI 已同步四份主规格、17 条新增需求，并归档到 openspec/changes/archive/2026-10-06-complete-wechat-upstream-capabilities。

## 收口补充

本次目标继续推进后，真实 Redis 与 release 并发验收均已补齐，双模块 consumer 已从仅编译转换增强为两个真实客户端的并发等价经营请求、响应及独立账号 token 验证。Rust 1.89 安装已依据 AGENTS.md 获得用户确认，并完成实际工作区全功能编译。验证证据与上游 JSON 清单已加入精确忽略例外，确保可以随交付保留。
