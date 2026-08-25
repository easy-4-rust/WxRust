//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.ExternalProductMappingResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalProductMappingResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 映射结果
    #[serde(rename = "mapping_list", default)]
    pub mapping_list: Vec<ExternalMappingInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalMappingInfo {
    /// 外部属性 ID
    #[serde(rename = "out_attr_id", default)]
    pub out_attr_id: String,
    /// 属性 ID
    #[serde(rename = "attr_id", default)]
    pub attr_id: String,
}
