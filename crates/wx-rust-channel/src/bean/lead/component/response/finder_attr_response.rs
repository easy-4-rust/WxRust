//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.response.FinderAttrResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::lead::component::*;
#[allow(unused_imports)]
use crate::bean::lead::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderAttrResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "finder_attr", default)]
    pub finder_attr: FinderAttr,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderAttr {
    #[serde(rename = "uniq_id", default)]
    pub uniq_id: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "fans_count", default)]
    pub fans_count: i32,
}
