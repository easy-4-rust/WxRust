# store-sdk Specification

## Purpose
提供可独立依赖和发布的微信小店 Rust SDK，使店铺经营业务能够脱离视频号模块接入，同时保留已有 Channel 使用者的公开接口、模型和运行行为以支持分阶段迁移。

## Requirements

### Requirement: 独立可用的小店模块
系统 SHALL 提供 wx-rust-store 独立 crate，具有真实服务实现、配置、模型和公开入口。仅依赖 store 的应用 MUST 能完成 token 获取及经营接口调用，其依赖树 MUST 不包含 wx-rust-channel。

#### Scenario: 单独接入
- **WHEN** 一个外部示例工程只声明 wx-rust-store
- **THEN** 可以构建并通过本地模拟服务执行认证与经营接口，依赖树不存在 Channel。

### Requirement: 固定基线的完整覆盖
Store SHALL 覆盖 WxJava 230ed0a 提交中 weixin-java-store 的全部公开经营接口与相关模型，包括店铺、类目、品牌、商品、仓库、订单、售后、物流、营销及资金。MUST 提供逐对象和逐方法覆盖清单；每项具备真实实现和对应测试，不以文件数量证明完成。

#### Scenario: 全量清单核验
- **WHEN** 将固定提交的完整模块树与 Rust 导出及方法清单比对
- **THEN** 无遗漏的公开经营接口、空实现或未验证条目；实现路径与验证证据可追溯。

### Requirement: 视频号能力边界
Store SHALL 不暴露视频号直播、Finder、联盟分销、留资组件或达人罗盘能力；这些能力 MUST 保留于 Channel。

#### Scenario: 两类业务共存
- **WHEN** 应用同时需要店铺经营及视频号业务
- **THEN** 可同时依赖 Store 和 Channel，分别从对应入口调用，配置和 token 不相互串用。

### Requirement: Channel 兼容迁移
旧 Channel 小店服务、类型路径、配置和序列化行为 SHALL 保持可用；Store SHALL 提供独立命名的新类型和明确的迁移说明。旧接口不在本次删除；弃用提示 MUST 仅覆盖已有 Store 替代的小店入口。

#### Scenario: 旧应用回归
- **WHEN** 现有 Channel 示例和接口测试使用原导入及配置构建运行
- **THEN** 通过相同接口发出等价请求，不要求调用方立即改代码。

#### Scenario: 分阶段迁移
- **WHEN** 应用只将订单业务切换到 Store
- **THEN** 其余 Channel 业务继续运行，文档说明新旧模型转换边界和服务/配置名称对应关系。

### Requirement: 可发布与可验证
Store SHALL 纳入工作区质量、打包及发布依赖顺序检查，并提供独立接入与迁移示例。MUST 对服务装配、认证隔离、错误响应及主要经营链路提供离线验证。

#### Scenario: 发布准备
- **WHEN** 执行 Store 打包检查和工作区回归
- **THEN** 包含所需源文件与元数据、无未声明本地依赖，原 Channel 回归和 Store 测试同时通过；未经实际执行的线上验证不得标记通过。
