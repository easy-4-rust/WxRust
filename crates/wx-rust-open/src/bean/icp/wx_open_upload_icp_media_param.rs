//! 小程序备案媒体材料上传参数。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenUploadIcpMediaParam`
//! （`@Data @Builder`，`@SerializedName` 线格式）。Rust 以普通结构表达
//! 同一数据模型。
//!
//! ADAPTED：`media` 字段类型 `CommonUploadData` 与 `toCommonUploadMultiParam`
//! 的结果类型 `CommonUploadMultiParam` 均无 serde 派生（multipart 载体），
//! 故本结构不派生 serde——上传组装在 Wave 2 上传执行器完成。

use crate::bean::{CommonUploadMultiParam, NormalParam};

/// 小程序备案媒体材料上传参数。
#[derive(Debug, Clone, Default)]
pub struct WxOpenUploadIcpMediaParam {
    /// 媒体材料类型。目前支持两种：图片（"image"）和视频（"video"），示例值："image"。
    pub ty: Option<String>,
    /// 证件类型（参考：获取证件类型），如果上传的是证件媒体材料，则必填，示例值：2。
    pub certificate_type: Option<i32>,
    /// 媒体材料所属的备案字段名（参考：申请小程序备案接口），如要用于多个备案字段，
    /// 则填写其中一个字段名即可。
    pub icp_order_field: Option<String>,
    /// 待上传的图片或视频。
    pub media: Option<wx_rust_common::bean::CommonUploadData>,
}

impl WxOpenUploadIcpMediaParam {
    /// 转换为多文件上传参数（对应 Java `toCommonUploadMultiParam()`）。
    ///
    /// Java 以 `String.valueOf(value)` 把 null 串化为 `"null"`，此处逐字对齐。
    pub fn to_common_upload_multi_param(&self) -> CommonUploadMultiParam {
        let value_of = |v: &Option<String>| v.clone().unwrap_or_else(|| "null".to_string());
        let value_of_int = |v: &Option<i32>| {
            v.map(|n| n.to_string())
                .unwrap_or_else(|| "null".to_string())
        };
        CommonUploadMultiParam {
            normal_params: vec![
                NormalParam {
                    name: "type".to_string(),
                    value: value_of(&self.ty),
                },
                NormalParam {
                    name: "certificate_type".to_string(),
                    value: value_of_int(&self.certificate_type),
                },
                NormalParam {
                    name: "icp_order_field".to_string(),
                    value: value_of(&self.icp_order_field),
                },
            ],
            upload_param: self.media.as_ref().map(|media| {
                wx_rust_common::bean::CommonUploadParam::new("media".to_string(), media.clone())
            }),
        }
    }
}
