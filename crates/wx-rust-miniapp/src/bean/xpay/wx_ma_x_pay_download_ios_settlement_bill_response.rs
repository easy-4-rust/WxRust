//! 下载 iOS 月结账单响应。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `download_ios_settlement_bill`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayDownloadIosSettlementBillResponse {
    /// 错误码，0 表示成功
    #[serde(rename = "errcode", default)]
    pub errcode: i32,

    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,

    /// 结算单列表
    #[serde(rename = "bill_list", default)]
    pub bill_list: Vec<WxMaXPayIosSettlementBill>,
}

impl WxMaXPayDownloadIosSettlementBillResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayDownloadIosSettlementBillResponse 序列化失败: {e}"))
    }
}
