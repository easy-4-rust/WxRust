//! 多文件上传参数集合。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.CommonUploadMultiParam`。Java 为
//! `@Data @Builder` 的 multipart 表单参数载体（`CommonUploadParam` 承载文件
//! 数据）；Rust 以普通结构表达同一数据模型。
//!
//! ADAPTED：`wx_rust_common::bean::CommonUploadParam`/`CommonUploadData`
//! 仅派生 `Debug, Clone`（无 serde/PartialEq/Default），故本结构与
//! `WxOpenUploadIcpMediaParam` 一样不派生 serde——multipart 组装在
//! 上传执行器（Wave 2）完成，不经过 JSON 线格式。

/// 多文件上传参数集合（对应 Java `CommonUploadMultiParam`）。
#[derive(Debug, Clone, Default)]
pub struct CommonUploadMultiParam {
    /// 普通表单参数列表（对应 Java `normalParams`）。
    pub normal_params: Vec<NormalParam>,
    /// 文件上传参数（对应 Java `uploadParam`）。
    pub upload_param: Option<wx_rust_common::bean::CommonUploadParam>,
}

/// 普通表单参数（对应 Java 内嵌类 `CommonUploadMultiParam.NormalParam`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NormalParam {
    /// 参数名称（非文件名），如：type。
    #[serde(rename = "name", default)]
    pub name: String,
    /// 参数名称对应值，如：image。
    #[serde(rename = "value", default)]
    pub value: String,
}
