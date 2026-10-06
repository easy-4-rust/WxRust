# Proposal

## Why

WxRust 0.1.4 已提供八个新增 XPay 网络接口，但仍缺少后续上游的个人主体虚拟支付数据生成、客服知识库、旅客运输电子发票及独立微信小店模块。用户已要求四项全部实施；P1/P2 只表示实施顺序，不表示可选范围。

本变更以本地 `260d800` 为起点，采用 OpenSpec 作为四项增量的唯一规格事实源。既有 Superpowers 文档保持历史参考，不迁移、不覆盖。

## What Changes

- P1：补齐个人虚拟支付的请求模型、前端支付参数生成、双签名及 JSON/XML 商品发货回调；复用现有 XPay，保留已有八个接口。
- P1：在现有企业微信客服服务中实现知识库分组及问答各四个接口，支持分页及完整附件模型。
- P1：实现旅客运输行业电子发票模型、开票方法、HTTP 202 空响应处理及敏感字段密文透传契约。
- P2：交付可独立使用的 `wx-rust-store` crate、服务与模型、测试和迁移指南；保留 Channel 的旧类型与行为，不能以设计文档、空模块或仅重导出 Channel 代替实现。
- 建立上游对象/字段/接口到 Rust 的覆盖清单，执行目标测试和工作区回归，更新核查报告与发布材料。
- 不执行线上交易、实际开票、自动发布或删除旧接口。不借本变更重构全部公共管线，也不承诺移植 Java Spring/Solon 框架适配器。

## Capabilities

### New Capabilities

- `miniapp-virtual-payment`: 个人主体虚拟支付数据与签名、商品发货回调兼容。
- `cp-kf-knowledge`: 企业微信客服知识库分组与问答管理。
- `pay-passenger-invoice`: 服务商旅客运输行业电子发票申请。
- `store-sdk`: 独立微信小店 SDK、旧 Channel 兼容及迁移。

### Modified Capabilities

无既有 OpenSpec 主规格；以上作为新增能力，不暗示现有 crate 不存在。

## Impact

涉及 miniapp 的 XPay/message、cp 的客服服务、pay 的 partner invoice、新增 store crate、workspace 依赖与发布清单、文档及测试。公共 trait 扩展需检查自定义实现兼容性；所有新增类型遵循一 Java 对象一 Rust 文件、中文来源注释与显式 import。

上游固定证据：

| 能力 | WxJava 提交 | 已核实内容 |
|---|---|---|
| 虚拟支付 | [0174215](https://github.com/binarywang/WxJava/commit/017421583aca8017aff5c5a1ba79fe58fd006bc2) | 四个新模型、本地生成方法、消息字段及 golden 测试 |
| 知识库 | [c1591ba](https://github.com/binarywang/WxJava/commit/c1591bae9c041ab82914d9129c10ee8cb08f9997) | 八个 POST 方法、六个顶层模型与测试 |
| 旅客发票 | [474f4c7](https://github.com/binarywang/WxJava/commit/474f4c7a0fc549032efad3c029fa9a981bd82f9a) | 请求模型、开票方法、空响应与序列化测试 |
| 微信小店 | [230ed0a](https://github.com/binarywang/WxJava/commit/230ed0a696855dc50a954331d8c0a023f15b3b0b) | 独立模块与迁移指南；提交文件列表超过 API 单页范围，实施前必须补齐完整树清单 |

规格与测试以固定提交及可核对的官方接口契约为依据，不将未核实的后续 develop 内容算作已覆盖。
