//! WxChannelAddressServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelAddressServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_address_service::WxChannelAddressService;
use crate::bean::address::{
    AddressAddParam, AddressDetail, AddressIdParam, AddressIdResponse, AddressInfoResponse,
    AddressListResponse,
};
use crate::bean::base::WxChannelBaseResponse;
use crate::enums::url_address as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 地址管理服务实现。
pub struct WxChannelAddressServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelAddressServiceImpl {
    /// 构建地址管理服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelAddressService for WxChannelAddressServiceImpl {
    /// 对应 Java `WxChannelAddressServiceImpl.listAddress`：
    /// `AddressListParam`（空值跳过，Java Jackson `NON_NULL`）后 POST
    /// `LIST_ADDRESS_URL`。
    async fn list_address(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AddressListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            (
                "offset",
                offset
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "limit",
                limit
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::LIST_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAddressServiceImpl.getAddress`：
    /// 序列化 `AddressIdParam` 后 POST `GET_ADDRESS_URL`。
    async fn get_address(
        &self,
        address_id: String,
    ) -> Result<AddressInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AddressIdParam { address_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAddressServiceImpl.addAddress`：
    /// 序列化 `AddressAddParam`（包裹 `AddressDetail`，key 为
    /// `address_detail`）后 POST `ADD_ADDRESS_URL`。
    async fn add_address(
        &self,
        address_detail: AddressDetail,
    ) -> Result<AddressIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AddressAddParam { address_detail };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAddressServiceImpl.updateAddress`：
    /// 序列化 `AddressAddParam` 后 POST `UPDATE_ADDRESS_URL`。
    async fn update_address_detail(
        &self,
        address_detail: AddressDetail,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AddressAddParam { address_detail };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelAddressServiceImpl.deleteAddress`：
    /// 序列化 `AddressIdParam` 后 POST `DELETE_ADDRESS_URL`。
    async fn delete_address(
        &self,
        address_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = AddressIdParam { address_id };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::DELETE_ADDRESS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
