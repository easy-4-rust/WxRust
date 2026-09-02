//! iOS 月结账单单条记录。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `download_ios_settlement_bill`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

/// 单个月份的结算单下载信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayIosSettlementBill {
    /// 月份，格式 YYYYMM。
    #[serde(rename = "month", default)]
    pub month: String,
    /// 账单下载链接，及时使用，一定时间后失效。
    #[serde(rename = "bill_url", default)]
    pub bill_url: String,
}

impl WxMaXPayIosSettlementBill {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayIosSettlementBill 序列化失败: {e}"))
    }
}
