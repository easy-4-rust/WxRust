//! 查询签约关系响应。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `query_subscribe_contract`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQuerySubscribeContractResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub errcode: i32,

    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,

    /// SIGNED: 签约生效中；TERMINATED: 已解约（终态）；UNBINDUSER: 从未签约过
    #[serde(rename = "authorization_state", default)]
    pub authorization_state: String,
}

impl WxMaXPayQuerySubscribeContractResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQuerySubscribeContractResponse 序列化失败: {e}"))
    }
}
