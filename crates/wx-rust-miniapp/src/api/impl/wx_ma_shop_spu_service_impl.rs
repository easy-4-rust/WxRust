//! 小程序交易组件-商品服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopSpuServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopSpuService;
use crate::bean::shop::WxMaShopSpuInfo;
use crate::bean::shop::WxMaShopSpuWithoutAuditInfo;
use crate::bean::shop::request::WxMaShopSpuPageRequest;
use crate::bean::shop::response::{
    WxMaShopAddSpuResponse, WxMaShopBaseResponse, WxMaShopGetSpuListResponse,
    WxMaShopGetSpuResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_spu as spu_url;

/// 构建 JSON 对象（跳过空值，对应 Java `GsonHelper.buildJsonObject`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 小程序交易组件-商品服务实现。
pub struct WxMaShopSpuServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopSpuServiceImpl {
    /// 构建商品服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopSpuService for WxMaShopSpuServiceImpl {
    /// 对应 Java `WxMaShopSpuServiceImpl.addSpu`：
    /// POST `SPU_ADD_URL`（序列化 `WxMaShopSpuInfo`）后解析响应。
    async fn add_spu(
        &self,
        spu_info: &WxMaShopSpuInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(spu_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&spu_url::spu_add_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.deleteSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `SPU_DEL_URL`。
    async fn delete_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&spu_url::spu_del_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.getSpu`：
    /// 构造 `{"product_id", "out_product_id", "need_edit_spu"}` 后 POST `SPU_GET_URL`。
    async fn get_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
        need_edit_spu: Option<i32>,
    ) -> Result<WxMaShopGetSpuResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "need_edit_spu",
                need_edit_spu
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&spu_url::spu_get_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.getSpuList`：
    /// POST `SPU_GET_LIST_URL`（序列化 `WxMaShopSpuPageRequest`）后解析响应。
    async fn get_spu_list(
        &self,
        request: &WxMaShopSpuPageRequest,
    ) -> Result<WxMaShopGetSpuListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&spu_url::spu_get_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.updateSpu`：
    /// POST `SPU_UPDATE_URL`（序列化 `WxMaShopSpuInfo`）后解析响应。
    async fn update_spu(
        &self,
        spu_info: &WxMaShopSpuInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(spu_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&spu_url::spu_update_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.updateSpuWithoutAudit`：
    /// POST `SPU_UPDATE_WITHOUT_URL`（序列化 `WxMaShopSpuWithoutAuditInfo`）后解析响应。
    async fn update_spu_without_audit(
        &self,
        spu_info: &WxMaShopSpuWithoutAuditInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(spu_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&spu_url::spu_update_without_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.listingSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `SPU_LISTING_URL`。
    async fn listing_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&spu_url::spu_listing_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.delistingSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `SPU_DELISTING_URL`。
    async fn delisting_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&spu_url::spu_delisting_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSpuServiceImpl.deleteAudit`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `DEL_AUDIT_URL`。
    async fn delete_audit(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&spu_url::spu_del_audit_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
