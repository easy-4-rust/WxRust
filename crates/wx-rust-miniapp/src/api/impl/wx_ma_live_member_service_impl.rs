//! 小程序直播成员管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaLiveMemberServiceImpl`：
//! `addRole`/`deleteRole` 返回微信原始响应报文；`listByRole` 返回 `list`
//! 节点 JSON 数组。

use std::collections::HashMap;
use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaLiveMemberService;
use crate::enums::g4_urls::url_g4_ability::live as live_url;

/// 小程序直播成员管理服务实现。
pub struct WxMaLiveMemberServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaLiveMemberServiceImpl {
    /// 构建直播成员管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 将键值对拼接为 query 串（对应 Java
    /// `Joiner.on("&").withKeyValueSeparator("=").join(map)`）。
    fn join_query(params: &HashMap<String, String>) -> String {
        let mut parts: Vec<String> = params.iter().map(|(k, v)| format!("{k}={v}")).collect();
        parts.sort();
        parts.join("&")
    }
}

#[async_trait]
impl WxMaLiveMemberService for WxMaLiveMemberServiceImpl {
    /// 设置成员角色（对应 Java `WxMaLiveMemberServiceImpl.addRole`，
    /// 返回微信原始响应报文）。
    async fn add_role(&self, username: &str, role: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "username": username, "role": role }).to_string();
        svc.post(&live_url::role::add_role_url(config.as_ref()), &post_body)
            .await
    }

    /// 解除成员角色（对应 Java `WxMaLiveMemberServiceImpl.deleteRole`，
    /// 返回微信原始响应报文）。
    async fn delete_role(&self, username: &str, role: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "username": username, "role": role }).to_string();
        svc.post(
            &live_url::role::delete_role_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 查询成员列表（对应 Java `WxMaLiveMemberServiceImpl.listByRole`，
    /// 返回 `list` 节点 JSON 数组；Java `keyword` 可为 null，Rust 以
    /// `Option` 表达，为 `None` 时不带该参数）。
    async fn list_by_role(
        &self,
        role: i32,
        offset: i32,
        limit: i32,
        keyword: Option<&str>,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let mut map = HashMap::new();
        map.insert("role".to_string(), role.to_string());
        map.insert("offset".to_string(), offset.to_string());
        map.insert("limit".to_string(), limit.to_string());
        if let Some(keyword) = keyword {
            map.insert("keyword".to_string(), keyword.to_string());
        }
        let query = Self::join_query(&map);
        let response = svc
            .get(&live_url::role::list_by_role_url(config.as_ref()), &query)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        json.get("list")
            .cloned()
            .ok_or_else(|| WxErrorException::from_code(-99, "list 字段缺失"))
    }
}
