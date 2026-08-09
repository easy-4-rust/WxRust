//! 对应 Java `bean.datacube.WxDataCubeArticleTotalDetail`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDataCubeArticleTotalDetail {
    #[serde(rename = "stat_date", default)]
    pub stat_date: String,
    #[serde(rename = "target_user", default)]
    pub target_user: i32,
    #[serde(rename = "int_page_read_user", default)]
    pub int_page_read_user: i32,
    #[serde(rename = "int_page_read_count", default)]
    pub int_page_read_count: i32,
    #[serde(rename = "ori_page_read_user", default)]
    pub ori_page_read_user: i32,
    #[serde(rename = "ori_page_read_count", default)]
    pub ori_page_read_count: i32,
    #[serde(rename = "share_user", default)]
    pub share_user: i32,
    #[serde(rename = "share_count", default)]
    pub share_count: i32,
    #[serde(rename = "add_to_fav_user", default)]
    pub add_to_fav_user: i32,
    #[serde(rename = "add_to_fav_count", default)]
    pub add_to_fav_count: i32,
    #[serde(rename = "int_page_from_session_read_user", default)]
    pub int_page_from_session_read_user: i32,
    #[serde(rename = "int_page_from_session_read_count", default)]
    pub int_page_from_session_read_count: i32,
    #[serde(rename = "int_page_from_hist_msg_read_user", default)]
    pub int_page_from_hist_msg_read_user: i32,
    #[serde(rename = "int_page_from_hist_msg_read_count", default)]
    pub int_page_from_hist_msg_read_count: i32,
    #[serde(rename = "int_page_from_feed_read_user", default)]
    pub int_page_from_feed_read_user: i32,
    #[serde(rename = "int_page_from_feed_read_count", default)]
    pub int_page_from_feed_read_count: i32,
    #[serde(rename = "int_page_from_friends_read_user", default)]
    pub int_page_from_friends_read_user: i32,
    #[serde(rename = "int_page_from_friends_read_count", default)]
    pub int_page_from_friends_read_count: i32,
    #[serde(rename = "int_page_from_other_read_user", default)]
    pub int_page_from_other_read_user: i32,
    #[serde(rename = "int_page_from_other_read_count", default)]
    pub int_page_from_other_read_count: i32,
    #[serde(rename = "feed_share_from_session_user", default)]
    pub feed_share_from_session_user: i32,
    #[serde(rename = "feed_share_from_session_cnt", default)]
    pub feed_share_from_session_cnt: i32,
    #[serde(rename = "feed_share_from_feed_user", default)]
    pub feed_share_from_feed_user: i32,
    #[serde(rename = "feed_share_from_feed_cnt", default)]
    pub feed_share_from_feed_cnt: i32,
    #[serde(rename = "feed_share_from_other_user", default)]
    pub feed_share_from_other_user: i32,
    #[serde(rename = "feed_share_from_other_cnt", default)]
    pub feed_share_from_other_cnt: i32,
    #[serde(rename = "int_page_from_kanyikan_read_user", default)]
    pub int_page_from_kanyikan_read_user: i32,
    #[serde(rename = "int_page_from_kanyikan_read_count", default)]
    pub int_page_from_kanyikan_read_count: i32,
    #[serde(rename = "int_page_from_souyisou_read_user", default)]
    pub int_page_from_souyisou_read_user: i32,
    #[serde(rename = "int_page_from_souyisou_read_count", default)]
    pub int_page_from_souyisou_read_count: i32,
}
