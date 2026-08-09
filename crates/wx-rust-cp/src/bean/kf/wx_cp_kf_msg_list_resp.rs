//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfMsgListResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMsgListResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "has_more", default)]
    pub has_more: i32,
    #[serde(rename = "msg_list", default)]
    pub msg_list: Vec<WxCpKfMsgItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMsgItem {
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "open_kfid", default)]
    pub open_kfid: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "send_time", default)]
    pub send_time: i64,
    #[serde(rename = "origin", default)]
    pub origin: i32,
    #[serde(rename = "servicer_userid", default)]
    pub servicer_user_id: String,
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::kf::msg::wx_cp_kf_text_msg::WxCpKfTextMsg,
    #[serde(rename = "image", default)]
    pub image: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "voice", default)]
    pub voice: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "video", default)]
    pub video: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "file", default)]
    pub file: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "location", default)]
    pub location: crate::bean::kf::msg::wx_cp_kf_location_msg::WxCpKfLocationMsg,
    #[serde(rename = "link", default)]
    pub link: crate::bean::kf::msg::wx_cp_kf_link_msg::WxCpKfLinkMsg,
    #[serde(rename = "business_card", default)]
    pub business_card: crate::bean::kf::msg::wx_cp_kf_business_card_msg::WxCpKfBusinessCardMsg,
    #[serde(rename = "miniprogram", default)]
    pub mini_program: crate::bean::kf::msg::wx_cp_kf_mini_program_msg::WxCpKfMiniProgramMsg,
    #[serde(rename = "msgmenu", default)]
    pub msg_menu: crate::bean::kf::msg::wx_cp_kf_menu_msg::WxCpKfMenuMsg,
    #[serde(rename = "event", default)]
    pub event: crate::bean::kf::msg::wx_cp_kf_event_msg::WxCpKfEventMsg,
    #[serde(rename = "channels_shop_product", default)]
    pub channels_shop_product:
        crate::bean::kf::msg::wx_cp_kf_channels_shop_product_msg::WxCpKfChannelsShopProductMsg,
    #[serde(rename = "channels_shop_order", default)]
    pub channels_shop_order:
        crate::bean::kf::msg::wx_cp_kf_channels_shop_order_msg::WxCpKfChannelsShopOrderMsg,
    #[serde(rename = "channels", default)]
    pub channels: crate::bean::kf::msg::wx_cp_kf_channels_msg::WxCpKfChannelsMsg,
}

impl WxCpKfMsgListResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfMsgListResp 解析失败: {e}"))
    }
}
