# Spec Delta

## Purpose

为微信支付服务商增加旅客运输行业电子发票申请能力，准确表达旅客、票据与交易信息，并明确受理、开票结果及敏感字段加密之间的边界，避免调用方误判业务成功。

## ADDED Requirements

### Requirement: 行业开票申请
SDK SHALL 通过现有服务商发票入口向 /v3/new-tax-control-fapiao/fapiao-applications/issue-passenger-transport 发送签名 POST，包含 sub_mchid、fapiao_apply_id、buyer_information、fapiao_information。

#### Scenario: 成功受理
- **WHEN** 合法申请获得 HTTP 202 且响应体为空
- **THEN** SDK 返回成功受理，不尝试解析空 JSON，也不将其表述为已经开票成功。

#### Scenario: 请求失败
- **WHEN** 微信支付返回业务失败或网络请求失败
- **THEN** SDK 返回现有支付错误类型，调用方可获取错误信息，不返回伪造的发票结果。

### Requirement: 完整票据与旅客信息
SDK SHALL 支持票据信息、发票行、交易信息、可选旅客信息，以及固定上游提交定义的所有行业字段。金额、数量、税率 MUST 使用能够保存 Java Long 范围的有符号整数；缺省可选字段省略。

#### Scenario: 大数量与折扣行
- **WHEN** quantity 为 2200000000，折扣行 total_amount 为负数
- **THEN** 序列化完整保存数值，不溢出、不转为浮点数或无符号数。

#### Scenario: 旅客字段
- **WHEN** 提供 name、certificate_type、certificate_number、departure_date、departure_place、destination、transportation_type 和 transportation_classes
- **THEN** 请求使用这些 snake_case 字段名并保持所有值，未提供旅客信息时省略整个对象。

### Requirement: 密文与证书标识一致
购买方手机号、邮箱和旅客证件号码 SHALL 按调用方预先加密的契约透传，SDK MUST 不对密文重复加密，不记录敏感完整请求体；请求使用的微信支付证书或公钥标识 MUST 与调用方加密依据一致。

#### Scenario: 密文透传
- **WHEN** 调用方提交预先加密的手机号、邮箱、证件号
- **THEN** 网络请求中的三个值与输入逐字节一致，发送链路不执行二次加密。

#### Scenario: 配置标识验证
- **WHEN** 使用测试证书或公钥标识完成请求
- **THEN** 可捕获请求中的相应 Wechatpay-Serial，并验证与加密依据匹配。

### Requirement: 保持现有发票能力
新增行业申请 SHALL 与普通开票、查询、冲红和回调能力共存；文档 MUST 指导调用方通过查询或通知确认最终结果。

#### Scenario: 受理后查询
- **WHEN** 申请成功受理后调用原查询接口
- **THEN** 原查询参数与返回行为保持不变，不需要更换服务配置。
