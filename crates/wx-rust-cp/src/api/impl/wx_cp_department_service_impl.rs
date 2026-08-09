//! 部门服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpDepartmentServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpDepartmentService, WxCpService};
use crate::bean::WxCpDepart;
use crate::enums::url_department::*;

/// 部门服务实现。
pub struct WxCpDepartmentServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpDepartmentServiceImpl {
    /// 构建部门服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpDepartmentService for WxCpDepartmentServiceImpl {
    async fn create(&self, depart: &WxCpDepart) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create`：POST `DEPARTMENT_CREATE`，请求体 `depart.toJson()`，
        // 响应取 `id`（GsonHelper.getAsLong）
        let config = svc.wx_cp_config_storage();
        let body = depart.to_json().map_err(WxErrorException::Serde)?;
        let response_content = svc.post(&config.api_url(DEPARTMENT_CREATE), &body).await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("id")
            .and_then(serde_json::Value::as_i64)
            .ok_or_else(|| WxErrorException::from_code(-99, "id 字段缺失"))
    }

    async fn get(&self, id: i64) -> Result<WxCpDepart, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get`：GET `String.format(DEPARTMENT_GET, id)`（`%d` 替换），
        // 响应取 `department` 子对象
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(DEPARTMENT_GET)
            .replace("%d", &id.to_string());
        let response_content = svc.get(&url, "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let depart = json
            .get("department")
            .ok_or_else(|| WxErrorException::from_code(-99, "department 字段缺失"))?;
        serde_json::from_value(depart.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn list(&self, id: Option<i64>) -> Result<Vec<WxCpDepart>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `list`：GET `DEPARTMENT_LIST`，`id` 非 null 时追加 `?id=`，
        // 响应取 `department` 数组
        let config = svc.wx_cp_config_storage();
        let mut url = config.api_url(DEPARTMENT_LIST);
        if let Some(id) = id {
            url.push_str(&format!("?id={id}"));
        }
        let response_content = svc.get(&url, "").await?;
        parse_department_list(&response_content, "department")
    }

    async fn simple_list(&self, id: Option<i64>) -> Result<Vec<WxCpDepart>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `simpleList`：GET `DEPARTMENT_SIMPLE_LIST`，`id` 非 null 时
        // 追加 `?id=`，响应取 `department_id` 数组（元素为 id 数字，逐个
        // 映射为仅含 `id` 的 `WxCpDepart`，ADAPTED：Java TypeToken 反序列化
        // 数字数组到 WxCpDepart 的实际行为即为该映射）
        let config = svc.wx_cp_config_storage();
        let mut url = config.api_url(DEPARTMENT_SIMPLE_LIST);
        if let Some(id) = id {
            url.push_str(&format!("?id={id}"));
        }
        let response_content = svc.get(&url, "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("department_id")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| WxErrorException::from_code(-99, "department_id 字段缺失"))?;
        let mut result = Vec::new();
        for v in list {
            if let Some(id) = v.as_i64() {
                result.push(WxCpDepart {
                    id: Some(id),
                    ..Default::default()
                });
            }
        }
        Ok(result)
    }

    async fn update(&self, group: &WxCpDepart) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：POST `DEPARTMENT_UPDATE`，请求体 `group.toJson()`
        let config = svc.wx_cp_config_storage();
        let body = group.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&config.api_url(DEPARTMENT_UPDATE), &body).await?;
        Ok(())
    }

    async fn delete(&self, depart_id: i64) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete`：GET `String.format(DEPARTMENT_DELETE, departId)`
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(DEPARTMENT_DELETE)
            .replace("%d", &depart_id.to_string());
        svc.get(&url, "").await?;
        Ok(())
    }
}

/// 解析响应中指定键的部门数组（对应 Java
/// `WxCpGsonBuilder.fromJson(tmpJsonObject.get(key), List<WxCpDepart>)`）。
fn parse_department_list(
    response_content: &str,
    key: &str,
) -> Result<Vec<WxCpDepart>, WxErrorException> {
    let json: serde_json::Value = serde_json::from_str(response_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
    let list = json
        .get(key)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| WxErrorException::from_code(-99, format!("{key} 字段缺失")))?;
    list.iter()
        .map(|v| {
            serde_json::from_value(v.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
        })
        .collect()
}
