//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SpuListResponse；对应 Java com.binarywang.wxjava.store.bean.product.SpuListResponse.java。
pub struct SpuListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "product_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}
