//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleDetail {
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "receive_product", default)]
    pub receive_product: bool,
    #[serde(rename = "cancel_time", default)]
    pub cancel_time: i64,
    #[serde(rename = "prove_imgs", default)]
    pub prove_imgs: Vec<String>,
    #[serde(rename = "tel_number", default)]
    pub tel_number: String,
    #[serde(rename = "media_id_list", default)]
    pub media_id_list: Vec<String>,
}
