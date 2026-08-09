//! 企业微信第三方应用部门服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpDepartmentServiceImpl`
//! （copy from `WxCpDepartmentServiceImpl`，唯一不同在于获取部门列表时
//! 需要传对应企业的 accessToken）：以 `Weak<dyn WxCpTpService>` 持有门面。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpTpDepart;
use crate::enums::url_department;
use crate::tp::service::{WxCpTpDepartmentService, WxCpTpService};

/// 企业微信第三方应用部门服务实现。
pub struct WxCpTpDepartmentServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpDepartmentServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }
}

#[async_trait]
impl WxCpTpDepartmentService for WxCpTpDepartmentServiceImpl {
    async fn create(&self, depart: &WxCpTpDepart) -> Result<i64, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_department::DEPARTMENT_CREATE);
        let json = depart.to_json().map_err(WxErrorException::Serde)?;
        let response_content = service.post(&url, &json).await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        tmp.get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "id 字段缺失"))
    }

    async fn update(&self, group: &WxCpTpDepart) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_department::DEPARTMENT_UPDATE);
        let json = group.to_json().map_err(WxErrorException::Serde)?;
        service.post(&url, &json).await?;
        Ok(())
    }

    async fn delete(&self, depart_id: i64) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java String.format(DEPARTMENT_DELETE, departId)
        let url = config
            .api_url(url_department::DEPARTMENT_DELETE)
            .replace("%d", &depart_id.to_string());
        service.get(&url, "").await?;
        Ok(())
    }

    async fn list(
        &self,
        id: Option<i64>,
        corp_id: &str,
    ) -> Result<Vec<WxCpTpDepart>, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let mut url = format!(
            "{}?access_token={}",
            config.api_url(url_department::DEPARTMENT_LIST),
            config.access_token(corp_id).unwrap_or_default()
        );
        if let Some(id) = id {
            url.push_str(&format!("&id={id}"));
        }
        let response_content = service.get(&url, "").await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = tmp
            .get("department")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        serde_json::from_value(list)
            .map_err(|e| WxErrorException::Serde(format!("department 列表解析失败: {e}")))
    }

    async fn list_all(&self, corp_id: &str) -> Result<Vec<WxCpTpDepart>, WxErrorException> {
        self.list(None, corp_id).await
    }
}
