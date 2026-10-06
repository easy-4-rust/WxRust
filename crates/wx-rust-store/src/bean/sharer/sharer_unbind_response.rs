//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.SharerUnbindResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SharerUnbindResponse；对应 Java com.binarywang.wxjava.store.bean.sharer.SharerUnbindResponse.java。
pub struct SharerUnbindResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "success_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_list: Option<Vec<String>>,
    #[serde(rename = "fail_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_list: Option<Vec<String>>,
    #[serde(rename = "refuse_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refuse_list: Option<Vec<String>>,
}
