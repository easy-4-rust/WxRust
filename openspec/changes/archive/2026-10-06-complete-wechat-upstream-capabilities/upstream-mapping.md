# 固定上游映射与验证入口

本变更固定四个 WxJava 提交，不依赖会移动的 develop HEAD。

| 目标 | 上游提交 | Rust 实现 | 行为验证 |
|---|---|---|---|
| U1 个人虚拟支付 | 017421583aca8017aff5c5a1ba79fe58fd006bc2 | miniapp/bean/xpay 四个 WxMaXPay 模型，api/wx_ma_xpay_service，message/wx_ma_message | xpay_virtual_payment_test：中文原文、两种 HMAC、缺省 attach、JSON/XML 回调；xpay_subscribe_download_test：原八接口 |
| U2 客服知识库 | c1591bae9c041ab82914d9129c10ee8cb08f9997 | cp/bean/kf 六个知识库模型及内部附件，api/wx_cp_kf_knowledge_service | kf_knowledge_test；wx_cp_kf_service_impl 的八端点、可选字段及错误测试 |
| U3 旅客电子发票 | 474f4c7a0fc549032efad3c029fa9a981bd82f9a | pay/bean/invoice/passenger_transport_invoice_request，api/passenger_transport_invoice_service，V3 202 成功分支 | partner_invoice_test：完整字段、i64 数量、负折扣、密文透传、签名头/公钥标识、202 空体、支付失败；原发票回归 |
| U4 独立 Store | 230ed0a696855dc50a954331d8c0a023f15b3b0b | wx-rust-store 独立 crate 与 26 个经营子服务 | store-coverage.json、上游字段 fixture、接口路径/错误契约、业务链路、服务装配、上传、认证隔离、独立/双模块 consumer |

## U1 对象

四个对象分别为 WxMaXPayRequestVirtualPaymentRequest、WxMaXPayRequestVirtualPaymentData、WxMaXPayGoodsInfo、WxMaXPayWeChatPayInfo，各有独立文件。支付请求的八个字段、前端数据的四个字段、商品的两个字段及支付信息的一个字段由 golden 与通知 fixture 验证。默认生成入口不增加网络请求。

## U2 对象与端点

六个顶层对象为 WxCpKfKnowledgeGroup、WxCpKfKnowledgeGroupAddResp、WxCpKfKnowledgeGroupListResp、WxCpKfKnowledgeIntent、WxCpKfKnowledgeIntentAddResp、WxCpKfKnowledgeIntentListResp。Intent 内部包含问题、相似问题、答案和五类附件，各 Java 内部类型保留在所属 Rust 文件。

分组和问答各有 add、del、mod、list，完整八端点见接入指南。查询 None 省略、显式空串保留；has_more 与 next_cursor 类型由响应 fixture 验证。

## U3 对象与字段

PassengerTransportInvoiceRequest 中保留 FapiaoInformation、InvoiceItem、PassengerInformation、TransactionInformation 四个内部类型，复用 BuyerInformation。全部可空字段为 Option，Java Long 为 i64。上游约束翻译到中文 doc；测试使用完整请求 fixture，包含数量 2200000000 和负折扣额。请求密文不再加密，接口复用既有 V3 签名、Wechatpay-Serial 与错误处理。

## U4 全量范围与适配说明

`store-upstream-tree.json` 保存完整树，truncated=false；643 个 main Java 对象均记录在 `store-coverage.json`，297 条公开方法/重载记录到具体 Rust 方法与测试引用。对象 SHA-256 可与固定源码再次核对。清单中的“Rust 适配”明确区分于原生类型迁移：Java HTTP 后端统一为 reqwest；Jackson/JSON/XML 工具改为 serde 与实际解析路径；Java Redis/Redisson 配置由 Rust 配置 trait 与内存配置提供接入边界，不声称包含同名第三方后端。

425 个可空/缺省场景测试确认 None 省略与显式空串保留。433 个普通数据对象的 fixture 直接从固定 Java 字段构造，与 Rust serde 声明独立；消息、setter 解包、HTTP 和 multipart 等行为使用专门测试。246 条接口路径/错误契约由固定 Java 方法和 URL 常量生成，额外业务测试覆盖请求字段、成功响应、金额、分页、回调及装配。上游明确返回“不支持”的 closeOrder 保留原行为，并验证不发送请求。

方法清单的测试引用是追溯入口，不能单独证明测试通过。最终运行结果以实施验证报告为准；未经真实账号联调的线上行为不列为通过。
