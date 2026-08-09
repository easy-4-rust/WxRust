//! 对应 Java `me.chanjar.weixin.open.bean.result.WxAmpLinkResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxAmpLinkResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "wxopens", default)]
    pub wx_open: WxOpen,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpen {
    #[serde(rename = "items", default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "username", default)]
    pub username: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "selected", default)]
    pub selected: i32,
    #[serde(rename = "nearby_display_status", default)]
    pub nearby_display_status: i32,
    #[serde(rename = "released", default)]
    pub released: i32,
    #[serde(rename = "headimg_url", default)]
    pub head_img_url: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "func_info", default)]
    pub func_info: Vec<FuncInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FuncInfo {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "id", default)]
    pub id: i64,
}
