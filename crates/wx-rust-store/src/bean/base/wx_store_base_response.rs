//! 对应 Java `com.binarywang.wxjava.store.bean.base.WxStoreBaseResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WxStoreBaseResponse；对应 Java com.binarywang.wxjava.store.bean.base.WxStoreBaseResponse.java。
pub struct WxStoreBaseResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}
