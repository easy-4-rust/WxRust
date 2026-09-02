//! 下载支付订单请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `start_download_order`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayStartDownloadOrderRequest {
    /// 开始日期，格式 YYYYMMDD
    #[serde(rename = "begin_ds", default)]
    pub begin_ds: i64,

    /// 结束日期，格式 YYYYMMDD，与 begin_ds 间隔不超过 31 天
    #[serde(rename = "end_ds", default)]
    pub end_ds: i64,

    /// 订单类型：1=代币交易 2=道具直购 3=会员订阅 4=退款订单
    #[serde(rename = "order_type", default)]
    pub order_type: i32,

    /// 订单信息搜索关键字，按交易单号/商户单号/用户ID 模糊匹配
    #[serde(rename = "order_info", default)]
    pub order_info: String,

    /// 发货状态，order_type 为 2/3 时必须传入；不传默认 true
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "is_provided",
        default
    )]
    pub is_provided: Option<bool>,

    /// 退款状态筛选，仅 order_type=4 有效；0=全部 2=已退款 4=退款中 5=退款失败
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "refund_status",
        default
    )]
    pub refund_status: Option<i32>,

    /// 环境标识：0=现网 1=沙箱
    #[serde(rename = "env", default)]
    pub env: i32,

    /// 支付渠道：1=普通虚拟支付 2=苹果IAP
    #[serde(rename = "pay_channel", default)]
    pub pay_channel: i32,
}

impl WxMaXPayStartDownloadOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayStartDownloadOrderRequest 序列化失败: {e}"))
    }
}
