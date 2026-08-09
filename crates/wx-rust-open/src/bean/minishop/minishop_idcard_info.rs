//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopIdcardInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopIdcardInfo {
    #[serde(rename = "idCardInfoId", default)]
    pub id_card_info_id: i32,
    #[serde(rename = "appId", default)]
    pub app_id: String,
    #[serde(rename = "portraitPicFile", default)]
    pub portrait_pic_file: MinishopPicFile,
    #[serde(rename = "protraitPicFileUrl", default)]
    pub protrait_pic_file_url: String,
    #[serde(rename = "nationPicFile", default)]
    pub nation_pic_file: MinishopPicFile,
    #[serde(rename = "nationPicFileUrl", default)]
    pub nation_pic_file_url: String,
    #[serde(rename = "idCardName", default)]
    pub id_card_name: String,
    #[serde(rename = "idCardNumber", default)]
    pub id_card_number: String,
    #[serde(rename = "startDate", default)]
    pub start_date: String,
    #[serde(rename = "endDate", default)]
    pub end_date: String,
}
