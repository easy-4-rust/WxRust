# Spec Delta

## Purpose

为个人主体小程序提供可直接交给微信前端的虚拟支付参数，并完整解析商品发货通知，使商户能够复用已有虚拟支付服务完成订单关联、签名校验和可靠的业务接入。

## ADDED Requirements

### Requirement: 生成完整前端支付数据
SDK SHALL 接受 offerId、buyQuantity、env、currencyType、productId、goodsPrice、outTradeNo、attach，返回 mode、signData、paySig、signature；此操作 MUST 为本地计算，不发起下单网络请求。

#### Scenario: 正常生成
- **WHEN** 调用方提供完整请求与 appKey、sessionKey
- **THEN** 返回 mode 为 short_series_goods，signData 包含按约定顺序序列化的请求，输出不包含 appKey 或 sessionKey。

#### Scenario: 可选字段缺省
- **WHEN** attach 未提供
- **THEN** signData 省略该字段，保持其他字段原值，不将缺省值转为字符串 null。

### Requirement: 双签名使用完全相同的字节
SDK SHALL 对同一份 UTF-8 signData 计算两个 HMAC-SHA256 小写十六进制签名：paySig 使用 appKey 和 requestVirtualPayment&signData；signature 使用 sessionKey 和 signData。MUST 保留中文、数值类型和字段顺序，不重新序列化签名输入。

#### Scenario: 上游固定向量
- **WHEN** signData 为 {"offerId":"1450019686","buyQuantity":1,"env":0,"currencyType":"CNY","productId":"product_001","goodsPrice":100,"outTradeNo":"order12345","attach":"attach中文"}，appKey 为 app_key_123，sessionKey 为 session_key_123
- **THEN** paySig 为 52b0abda3c933b0273d328b5c5102ee9ab9249309474ddcdf5e9ce0f80b23532，signature 为 602f76d9cadf36c232f7f6c1faa5fe295b0f955d188049997d0df699c0619c86。

### Requirement: 商品发货通知兼容
SDK SHALL 从 JSON 和 XML 通知解析 OutTradeNo、WeChatPayInfo.MchOrderNo、GoodsInfo.ProductId 与 GoodsInfo.Quantity，并保留原消息类型、重试次数及其他既有字段。新增字段缺省 MUST 不影响旧通知。

#### Scenario: 两种格式内容等价
- **WHEN** 同一 xpay_goods_deliver_notify 分别以 JSON 和 XML 输入，订单号为 order12345，微信单号为 wx_order_123，商品为 product_001，数量为 2
- **THEN** 两种输入得到等价的订单和商品信息。

#### Scenario: 旧消息回归
- **WHEN** 旧消息没有上述新增字段
- **THEN** 原解析成功，新增字段为缺省状态，已有八个 XPay 网络接口保持原行为。
