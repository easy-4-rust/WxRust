//! 小程序代码包提交审核消息（仅供第三方开发者代小程序调用）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.message.WxOpenMaSubmitAuditMessage`
//! （`@SerializedName` 线格式）。
//!
//! ADAPTED：Java 引用 `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditItem`
//! 与 `WxMaCodeSubmitAuditPreviewInfo`（miniapp 模块类型）。Rust 侧 Wave 1
//! 在 message 模块内镜像同线格式类型（字段与 `@SerializedName` 逐一对齐，
//! 参见 weixin-java-miniapp 同名列）；Wave 2 引入 wx-rust-miniapp 依赖后
//! 改为复用 `wx_rust_miniapp::bean::code::*` 类型。

/// 小程序代码包提交审核消息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaSubmitAuditMessage {
    /// 提交审核项的一个列表（至少填写1项，至多填写5项）。
    #[serde(rename = "item_list", default)]
    pub item_list: Option<Vec<WxMaCodeSubmitAuditItem>>,
    /// 预览信息（小程序页面截图和操作录屏）。
    #[serde(rename = "preview_info", default)]
    pub preview_info: Option<WxMaCodeSubmitAuditPreviewInfo>,
    /// 小程序版本说明和功能解释。
    #[serde(rename = "version_desc", default)]
    pub version_desc: Option<String>,
    /// 反馈内容，不超过200字。
    #[serde(rename = "feedback_info", default)]
    pub feedback_info: Option<String>,
    /// 图片media_id列表，中间用“丨”分割，xx丨yy丨zz，不超过5张图片。
    #[serde(rename = "feedback_stuff", default)]
    pub feedback_stuff: Option<String>,
    /// 用于声明是否不使用“代码中检测出但是未配置的隐私相关接口”。
    #[serde(rename = "privacy_api_not_use", default)]
    pub privacy_api_not_use: Option<bool>,
    /// 订单中心path。
    #[serde(rename = "order_path", default)]
    pub order_path: Option<String>,
}

/// 小程序帐号的可选类目提交项（镜像 Java
/// `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditItem`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditItem {
    /// 小程序的页面，可通过“获取小程序的第三方提交代码的页面配置”接口获得。
    #[serde(rename = "address", default)]
    pub address: Option<String>,
    /// 小程序的标签，多个标签用空格分隔，标签不能多于10个，标签长度不超过20。
    #[serde(rename = "tag", default)]
    pub tag: Option<String>,
    /// 一级类目名称。
    #[serde(rename = "first_class", default)]
    pub first_class: Option<String>,
    /// 二级类目名称。
    #[serde(rename = "second_class", default)]
    pub second_class: Option<String>,
    /// 三级类目名称。
    #[serde(rename = "third_class", default)]
    pub third_class: Option<String>,
    /// 一级类目的ID编号。
    #[serde(rename = "first_id", default)]
    pub first_id: Option<i64>,
    /// 二级类目的ID编号。
    #[serde(rename = "second_id", default)]
    pub second_id: Option<i64>,
    /// 三级类目的ID编号。
    #[serde(rename = "third_id", default)]
    pub third_id: Option<i64>,
    /// 小程序页面的标题,标题长度不超过32。
    #[serde(rename = "title", default)]
    pub title: Option<String>,
}

/// 预览信息（镜像 Java
/// `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditPreviewInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditPreviewInfo {
    /// 录屏mediaid列表，可以通过提审素材上传接口获得。
    #[serde(rename = "video_id_list", default)]
    pub video_id_list: Option<Vec<String>>,
    /// 截屏mediaid列表，可以通过提审素材上传接口获得。
    #[serde(rename = "pic_id_list", default)]
    pub pic_id_list: Option<Vec<String>>,
}
