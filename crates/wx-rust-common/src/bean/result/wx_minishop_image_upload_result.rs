//! 对应 Java `me.chanjar.weixin.common.bean.result.WxMinishopImageUploadResult`（由 gen_bean_structs.py 生成）。

use super::wx_minishop_pic_file_result::WxMinishopPicFileResult;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopImageUploadResult {
    /// errcode
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    /// errmsg
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    /// picFile
    #[serde(rename = "picFile", default)]
    pub pic_file: WxMinishopPicFileResult,
}
