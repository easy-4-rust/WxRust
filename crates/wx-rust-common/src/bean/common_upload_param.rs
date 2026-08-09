//! 通用文件上传参数。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.CommonUploadParam`。

use super::CommonUploadData;

/// 通用文件上传参数。
///
/// 用于通用上传接口（`WxService::upload`）的参数载体。
#[derive(Debug, Clone)]
pub struct CommonUploadParam {
    /// 文件对应的接口参数名称（非文件名），如 `media`
    pub name: String,

    /// 上传数据
    pub data: CommonUploadData,

    /// 额外的表单字段，用于在上传文件的同时提交其他表单数据。
    ///
    /// 例如上传视频素材时需要提交 `description` 字段（JSON 格式的视频描述信息）。
    pub form_fields: Option<std::collections::HashMap<String, String>>,
}

impl CommonUploadParam {
    /// 构建通用上传参数。
    ///
    /// # 参数
    /// - `name`：文件对应的接口参数名称（非文件名），如 `media`
    /// - `data`：上传数据
    pub fn new(name: impl Into<String>, data: CommonUploadData) -> Self {
        Self {
            name: name.into(),
            data,
            form_fields: None,
        }
    }

    /// 构建带额外表单字段的上传参数。
    ///
    /// # 参数
    /// - `name`：文件对应的接口参数名称
    /// - `data`：上传数据
    /// - `form_fields`：额外的表单字段
    pub fn with_form_fields(
        name: impl Into<String>,
        data: CommonUploadData,
        form_fields: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            name: name.into(),
            data,
            form_fields: Some(form_fields),
        }
    }
}
