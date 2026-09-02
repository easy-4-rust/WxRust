//! 下载虚拟支付 iOS 月结账单请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `download_ios_settlement_bill`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayDownloadIosSettlementBillRequest {
    /// 开始月份，格式 YYYYMM
    #[serde(rename = "start_month", default)]
    pub start_month: String,

    /// 结束月份，格式 YYYYMM
    #[serde(rename = "end_month", default)]
    pub end_month: String,
}

impl WxMaXPayDownloadIosSettlementBillRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayDownloadIosSettlementBillRequest 序列化失败: {e}"))
    }
}
