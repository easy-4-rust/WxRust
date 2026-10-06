//! WxStoreCouponServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreCouponServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_coupon_service::WxStoreCouponService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::coupon::{
    CouponIdInfo, CouponIdResponse, CouponInfoResponse, CouponListParam, CouponListResponse,
    CouponParam, UserCouponIdParam, UserCouponListParam, UserCouponListResponse,
    UserCouponResponse,
};
use crate::enums::url_coupon as url;

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

/// 优惠券服务实现。
pub struct WxStoreCouponServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreCouponServiceImpl {
    /// 构建优惠券服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreCouponService for WxStoreCouponServiceImpl {
    /// 对应 Java `WxStoreCouponServiceImpl.createCoupon`：
    /// 序列化 `CouponParam` 后 POST `CREATE_COUPON_URL`。
    async fn create_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&coupon).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::CREATE_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.updateCoupon`：
    /// 序列化 `CouponParam` 后 POST `UPDATE_COUPON_URL`。
    async fn update_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&coupon).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.updateCouponStatus`：
    /// `CouponStatusParam`（`status` 空值跳过）后 POST
    /// `UPDATE_COUPON_STATUS_URL`。
    async fn update_coupon_status(
        &self,
        coupon_id: String,
        status: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            ("coupon_id", serde_json::Value::String(coupon_id)),
            (
                "status",
                status
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::UPDATE_COUPON_STATUS_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.getCoupon`：
    /// 序列化 `CouponIdInfo` 后 POST `GET_COUPON_URL`。
    async fn get_coupon(&self, coupon_id: String) -> Result<CouponInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = CouponIdInfo {
            coupon_id: Some(coupon_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.getCouponList`：
    /// 序列化 `CouponListParam` 后 POST `LIST_COUPON_URL`。
    async fn get_coupon_list(
        &self,
        param: CouponListParam,
    ) -> Result<CouponListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::LIST_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.getUserCoupon`：
    /// 序列化 `UserCouponIdParam` 后 POST `GET_USER_COUPON_URL`。
    async fn get_user_coupon(
        &self,
        open_id: String,
        user_coupon_id: String,
    ) -> Result<UserCouponResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = UserCouponIdParam {
            openid: Some(open_id),
            user_coupon_id: Some(user_coupon_id),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_USER_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreCouponServiceImpl.getUserCouponList`：
    /// 序列化 `UserCouponListParam` 后 POST `LIST_USER_COUPON_URL`。
    async fn get_user_coupon_list(
        &self,
        param: UserCouponListParam,
    ) -> Result<UserCouponListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::LIST_USER_COUPON_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
