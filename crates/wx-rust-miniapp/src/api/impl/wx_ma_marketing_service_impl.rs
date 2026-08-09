//! 微信营销服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaMarketingServiceImpl`：
//! `addUserAction` 的请求体按 Java `WxMaUserAction.listToJson` 手工组装
//! （`action_time`/`action_type` 下划线键、`trace.click_id`、
//! `action_param.value/leads_type` 嵌套结构，bean 的平铺序列化不适用）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaMarketingService;
use crate::bean::marketing::WxMaUserAction;
use crate::enums::g4_urls::url_g4_ability::marketing as marketing_url;

/// 微信营销服务实现。
pub struct WxMaMarketingServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaMarketingServiceImpl {
    /// 构建营销服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaMarketingService for WxMaMarketingServiceImpl {
    /// 创建数据源（对应 Java `WxMaMarketingServiceImpl.addUserActionSets`）。
    ///
    /// 响应解析 `data.user_action_set_id`（Java
    /// `tmpJson.get("data").getAsJsonObject().get("user_action_set_id").getAsLong()`）。
    async fn add_user_action_sets(
        &self,
        r#type: &str,
        name: &str,
        description: &str,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = serde_json::json!({
            "type": r#type,
            "name": name,
            "description": description,
        })
        .to_string();
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &marketing_url::user_action_sets_add_url(config.as_ref()),
                &post_body,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("data")
            .and_then(|d| d.get("user_action_set_id"))
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "data.user_action_set_id 字段缺失"))
    }

    /// 回传数据（对应 Java `WxMaMarketingServiceImpl.addUserAction`，返回
    /// 微信原始响应报文）。
    ///
    /// 请求体按 Java `WxMaUserAction.listToJson` 组装：
    /// `{"user_action_set_id":..., "actions":[{url, action_time, action_type,
    /// trace:{click_id}?, action_param:{value, leads_type?}?}]}`；
    /// clickId/actionParam/leadsType 为 null（Rust 空串）时不携带。
    async fn add_user_action(
        &self,
        actions: &[WxMaUserAction],
        user_action_set_id: Option<i64>,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut action_array = Vec::new();
        for action in actions {
            let mut json = serde_json::Map::new();
            json.insert("url".to_string(), serde_json::json!(action.url));
            json.insert(
                "action_time".to_string(),
                serde_json::json!(action.action_time),
            );
            json.insert(
                "action_type".to_string(),
                serde_json::json!(action.action_type),
            );
            if !action.click_id.is_empty() {
                let mut trace = serde_json::Map::new();
                trace.insert("click_id".to_string(), serde_json::json!(action.click_id));
                json.insert("trace".to_string(), serde_json::Value::Object(trace));
            }
            if action.action_param != 0 {
                let mut action_param_json = serde_json::Map::new();
                action_param_json
                    .insert("value".to_string(), serde_json::json!(action.action_param));
                if !action.leads_type.is_empty() {
                    action_param_json.insert(
                        "leads_type".to_string(),
                        serde_json::json!(action.leads_type),
                    );
                }
                json.insert(
                    "action_param".to_string(),
                    serde_json::Value::Object(action_param_json),
                );
            }
            action_array.push(serde_json::Value::Object(json));
        }
        let post_body = serde_json::json!({
            "user_action_set_id": user_action_set_id.map(|id| serde_json::json!(id)).unwrap_or(serde_json::Value::Null),
            "actions": action_array,
        })
        .to_string();
        let config = svc.wx_ma_config();
        svc.post(
            &marketing_url::user_actions_add_url(config.as_ref()),
            &post_body,
        )
        .await
    }
}
