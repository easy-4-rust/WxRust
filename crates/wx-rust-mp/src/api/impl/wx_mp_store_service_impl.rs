//! WxMpStoreService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpStoreServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpStoreService};

use crate::bean::store::{WxMpStoreBaseInfo, WxMpStoreListResult};
use crate::enums::wx_mp_api_url::store;

/// WxMpStore服务实现。
pub struct WxMpStoreServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpStoreServiceImpl {
    /// 构建 WxMpStore服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpStoreService for WxMpStoreServiceImpl {
    async fn add(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = request.to_json();
        svc.post(&store::poi_add(config.as_ref()), &body).await?;
        Ok(())
    }

    async fn get(&self, poi_id: &str) -> Result<WxMpStoreBaseInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"poi_id": poi_id});
        let response = svc
            .post(&store::poi_get(config.as_ref()), &body.to_string())
            .await?;
        // Java 语义：取 business.base_info 子对象解析
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let base = value
            .get("business")
            .and_then(|b| b.get("base_info"))
            .ok_or_else(|| WxErrorException::from_code(-99, "business.base_info 缺失"))?;
        serde_json::from_value(base.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn delete(&self, poi_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"poi_id": poi_id});
        svc.post(&store::poi_del(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    async fn list(&self, begin: i32, limit: i32) -> Result<WxMpStoreListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"begin": begin, "limit": limit});
        let response = svc
            .post(&store::poi_list(config.as_ref()), &body.to_string())
            .await?;
        WxMpStoreListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = request.to_json();
        svc.post(&store::poi_update(config.as_ref()), &body).await?;
        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<String>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc.get(&store::wx_category(config.as_ref()), "").await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("category_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .ok_or_else(|| WxErrorException::from_code(-99, "category_list 缺失"))
    }
}
