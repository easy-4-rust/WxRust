//! 对应 Java `me.chanjar.weixin.channel.bean.favorite.FavoriteCountResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FavoriteCountResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 收藏人数
    #[serde(rename = "favorite_count", default)]
    pub favorite_count: i64,
}
