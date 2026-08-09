//! 小程序 URL Link 二维码快速跳转规则管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaQrcodeJumpServiceImpl`：
//! `addRule`/`deleteRule` 返回微信原始响应报文；`getRules`/`getRuleList`
//! 请求体仅携带非 null 字段，响应经 `WxMaQrcodeJumpRuleListResponse`
//! 解析，规则列表为空时返回空列表（Java `Collections.emptyList()`）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaQrcodeJumpService;
use crate::bean::qrcode::WxMaQrcodeJumpRule;
use crate::enums::g4_urls::url_g4_ability::qrcode_jump as qrcode_jump_url;

/// 小程序 URL Link 二维码快速跳转规则管理服务实现。
pub struct WxMaQrcodeJumpServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaQrcodeJumpServiceImpl {
    /// 构建二维码快速跳转规则管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaQrcodeJumpService for WxMaQrcodeJumpServiceImpl {
    /// 添加二维码快速跳转规则（对应 Java
    /// `WxMaQrcodeJumpServiceImpl.addRule`，返回原始响应报文）。
    async fn add_rule(&self, rule: &WxMaQrcodeJumpRule) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = serde_json::to_string(rule).map_err(WxErrorException::from)?;
        let config = svc.wx_ma_config();
        svc.post(&qrcode_jump_url::add_rule_url(config.as_ref()), &post_body)
            .await
    }

    /// 获取二维码快速跳转规则（对应 Java
    /// `WxMaQrcodeJumpServiceImpl.getRules`）。
    ///
    /// 请求体仅携带非 null 的 `is_default`/`prefix`；规则列表为空时返回
    /// 空列表（Java `Collections.emptyList()`）。
    async fn get_rules(
        &self,
        is_default: Option<bool>,
        prefix: Option<&str>,
    ) -> Result<Vec<WxMaQrcodeJumpRule>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut request = serde_json::Map::new();
        if let Some(is_default) = is_default {
            request.insert("is_default".to_string(), serde_json::json!(is_default));
        }
        if let Some(prefix) = prefix {
            request.insert("prefix".to_string(), serde_json::json!(prefix));
        }
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &qrcode_jump_url::get_rules_url(config.as_ref()),
                &serde_json::Value::Object(request).to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        let result: crate::bean::qrcode::WxMaQrcodeJumpRuleListResponse =
            serde_json::from_value(json).map_err(WxErrorException::from)?;
        if result.rule_list.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(result.rule_list)
        }
    }

    /// 分页获取二维码快速跳转规则列表（对应 Java
    /// `WxMaQrcodeJumpServiceImpl.getRuleList`）。
    ///
    /// 请求体仅携带非 null 的 `get_type`/`page_num`/`page_size`；规则列表为
    /// 空时返回空列表（Java `Collections.emptyList()`）。
    async fn get_rule_list(
        &self,
        get_type: Option<i32>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Vec<WxMaQrcodeJumpRule>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut request = serde_json::Map::new();
        if let Some(get_type) = get_type {
            request.insert("get_type".to_string(), serde_json::json!(get_type));
        }
        if let Some(page_num) = page_num {
            request.insert("page_num".to_string(), serde_json::json!(page_num));
        }
        if let Some(page_size) = page_size {
            request.insert("page_size".to_string(), serde_json::json!(page_size));
        }
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &qrcode_jump_url::get_rule_list_url(config.as_ref()),
                &serde_json::Value::Object(request).to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        let result: crate::bean::qrcode::WxMaQrcodeJumpRuleListResponse =
            serde_json::from_value(json).map_err(WxErrorException::from)?;
        if result.rule_list.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(result.rule_list)
        }
    }

    /// 删除二维码快速跳转规则（对应 Java
    /// `WxMaQrcodeJumpServiceImpl.deleteRule`，返回原始响应报文）。
    async fn delete_rule(&self, prefix: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = serde_json::json!({ "prefix": prefix }).to_string();
        let config = svc.wx_ma_config();
        svc.post(
            &qrcode_jump_url::delete_rule_url(config.as_ref()),
            &post_body,
        )
        .await
    }
}
