//! 对应 Java `me.chanjar.weixin.common.bean.result.WxMinishopImageUploadCustomizeResult`（由 gen_bean_structs.py 生成）。

use super::wx_minishop_pic_file_customize_result::WxMinishopPicFileCustomizeResult;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopImageUploadCustomizeResult {
    /// errcode
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    /// errmsg
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    /// imgInfo
    #[serde(rename = "imgInfo", default)]
    pub img_info: WxMinishopPicFileCustomizeResult,
}
