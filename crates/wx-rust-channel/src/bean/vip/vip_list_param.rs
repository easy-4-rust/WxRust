//! 对应 Java `me.chanjar.weixin.channel.bean.vip.VipListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VipListParam {
    #[serde(rename = "need_phone_number", default)]
    pub need_phone_number: bool,
    #[serde(rename = "page_num", default)]
    pub page_num: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
}
