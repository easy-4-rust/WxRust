//! 对应 Java `me.chanjar.weixin.channel.bean.vip.VipInfoParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VipInfoParam {
    #[serde(rename = "openid", default)]
    pub open_id: String,
    #[serde(rename = "need_phone_number", default)]
    pub need_phone_number: bool,
}
