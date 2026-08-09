//! 小程序 URL Link 二维码快速跳转规则管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaQrcodeJumpService`
//! （`impl.WxMaQrcodeJumpServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::qrcode::WxMaQrcodeJumpRule;

/// 小程序 URL Link 二维码快速跳转规则管理服务。
///
/// 对应 Java `WxMaQrcodeJumpService`：添加/获取/分页获取/删除二维码快速跳转规则。
#[async_trait]
pub trait WxMaQrcodeJumpService: Send + Sync {
    /// 添加二维码快速跳转规则（对应 Java `addRule`，返回原始响应报文）。
    async fn add_rule(&self, rule: &WxMaQrcodeJumpRule) -> Result<String, WxErrorException>;

    /// 获取二维码快速跳转规则（对应 Java `getRules`）。
    async fn get_rules(
        &self,
        is_default: Option<bool>,
        prefix: Option<&str>,
    ) -> Result<Vec<WxMaQrcodeJumpRule>, WxErrorException>;

    /// 分页获取二维码快速跳转规则列表（对应 Java `getRuleList`）。
    async fn get_rule_list(
        &self,
        get_type: Option<i32>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Vec<WxMaQrcodeJumpRule>, WxErrorException>;

    /// 删除二维码快速跳转规则（对应 Java `deleteRule`，返回原始响应报文）。
    async fn delete_rule(&self, prefix: &str) -> Result<String, WxErrorException>;
}
