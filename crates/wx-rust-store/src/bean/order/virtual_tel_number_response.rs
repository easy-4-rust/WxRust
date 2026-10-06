//! 对应 Java `com.binarywang.wxjava.store.bean.order.VirtualTelNumberResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 VirtualTelNumberResponse；对应 Java com.binarywang.wxjava.store.bean.order.VirtualTelNumberResponse.java。
pub struct VirtualTelNumberResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "virtual_tel_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_tel_number: Option<String>,
    #[serde(rename = "virtual_tel_expire_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_tel_expire_time: Option<i64>,
    #[serde(rename = "get_virtual_tel_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_virtual_tel_cnt: Option<i32>,
}
