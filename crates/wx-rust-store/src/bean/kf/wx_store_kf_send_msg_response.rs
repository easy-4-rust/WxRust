//! 对应 Java `com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WxStoreKfSendMsgResponse；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgResponse.java。
pub struct WxStoreKfSendMsgResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 上游字段 msg_id。
    #[serde(rename = "msg_id", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
}
