//! 小程序交易组件-优惠券服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopCouponServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopCouponService;
use crate::bean::shop::WxMaShopCouponInfo;
use crate::bean::shop::response::{
    WxMaShopBaseResponse, WxMaShopCouponListResponse, WxMaShopCouponResponse,
    WxMaShopUserCouponListResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_coupon as coupon_url;

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

/// 小程序交易组件-优惠券服务实现。
pub struct WxMaShopCouponServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopCouponServiceImpl {
    /// 构建优惠券服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopCouponService for WxMaShopCouponServiceImpl {
    /// 对应 Java `WxMaShopCouponServiceImpl.addCoupon`：
    /// 构造 `{"coupon": couponInfo}` 后 POST `ADD_COUPON`。
    async fn add_coupon(
        &self,
        coupon_info: &WxMaShopCouponInfo,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let coupon = serde_json::to_value(coupon_info)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = build_json(&[("coupon", coupon)]);
        let response = svc
            .post(&coupon_url::add_coupon_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.getCoupon`：
    /// 构造 `{"out_coupon_id": outCouponId}` 后 POST `GET_COUPON`。
    async fn get_coupon(
        &self,
        out_coupon_id: &str,
    ) -> Result<WxMaShopCouponResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "out_coupon_id",
            serde_json::Value::String(out_coupon_id.to_string()),
        )]);
        let response = svc
            .post(&coupon_url::get_coupon_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.getCouponList`：
    /// 构造 `{"page_size", "offset"}` 后 POST `GET_COUPON_LIST`。
    async fn get_coupon_list(
        &self,
        page_size: Option<i32>,
        offset: Option<i32>,
    ) -> Result<WxMaShopCouponListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "offset",
                offset
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&coupon_url::get_coupon_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.updateCoupon`：
    /// 构造 `{"coupon": couponInfo}` 后 POST `UPDATE_COUPON`。
    async fn update_coupon(
        &self,
        coupon_info: &WxMaShopCouponInfo,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let coupon = serde_json::to_value(coupon_info)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = build_json(&[("coupon", coupon)]);
        let response = svc
            .post(&coupon_url::update_coupon_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.updateCouponStatus`：
    /// 构造 `{"out_coupon_id", "status"}` 后 POST `UPDATE_COUPON_STATUS`。
    async fn update_coupon_status(
        &self,
        out_coupon_id: &str,
        status: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "out_coupon_id",
                serde_json::Value::String(out_coupon_id.to_string()),
            ),
            ("status", serde_json::Value::from(status)),
        ]);
        let response = svc
            .post(
                &coupon_url::update_coupon_status_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.updateCouponStock`：
    /// 构造 `{"coupon_stock": {"out_coupon_id", "stock_info": {"issued_num", "receive_num"}}}`
    /// 后 POST `UPDATE_COUPON_STOCK`。
    async fn update_coupon_stock(
        &self,
        out_coupon_id: &str,
        is_used_num: i32,
        receive_num: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let stock_info = serde_json::json!({
            "issued_num": is_used_num,
            "receive_num": receive_num,
        });
        let stock = build_json(&[
            (
                "out_coupon_id",
                serde_json::Value::String(out_coupon_id.to_string()),
            ),
            ("stock_info", stock_info),
        ]);
        let stock_value: serde_json::Value =
            serde_json::from_str(&stock).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = build_json(&[("coupon_stock", stock_value)]);
        let response = svc
            .post(&coupon_url::update_coupon_stock_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.addUserCoupon`：
    /// 构造 `{"openid", "user_coupon": {"out_user_coupon_id", "out_coupon_id", "status"},
    /// "recv_time"}` 后 POST `ADD_USER_COUPON`。
    async fn add_user_coupon(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        status: i32,
        recv_time: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let user_coupon = build_json(&[
            (
                "out_user_coupon_id",
                serde_json::Value::String(out_user_coupon_id.to_string()),
            ),
            (
                "out_coupon_id",
                serde_json::Value::String(out_coupon_id.to_string()),
            ),
            ("status", serde_json::Value::from(status)),
        ]);
        let user_coupon_value: serde_json::Value = serde_json::from_str(&user_coupon)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = build_json(&[
            ("openid", serde_json::Value::String(openid.to_string())),
            ("user_coupon", user_coupon_value),
            (
                "recv_time",
                recv_time
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&coupon_url::add_user_coupon_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.getUserCouponList`：
    /// 构造 `{"page_size", "offset", "openid"}` 后 POST `GET_USER_COUPON_LIST`。
    async fn get_user_coupon_list(
        &self,
        page_size: Option<i32>,
        offset: Option<i32>,
        openid: &str,
    ) -> Result<WxMaShopUserCouponListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "offset",
                offset
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            ("openid", serde_json::Value::String(openid.to_string())),
        ]);
        let response = svc
            .post(
                &coupon_url::get_user_coupon_list_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.updateUserCoupon`：
    /// 构造 `{"openid", "user_coupon": {"out_user_coupon_id", "out_coupon_id",
    /// "ext_info": {"use_time"}}, "recv_time"}` 后 POST `UPDATE_USER_COUPON`。
    async fn update_user_coupon(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        use_time: Option<i64>,
        recv_time: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let ext_info = build_json(&[(
            "use_time",
            use_time
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        )]);
        let ext_info_value: serde_json::Value =
            serde_json::from_str(&ext_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let user_coupon = build_json(&[
            (
                "out_user_coupon_id",
                serde_json::Value::String(out_user_coupon_id.to_string()),
            ),
            (
                "out_coupon_id",
                serde_json::Value::String(out_coupon_id.to_string()),
            ),
            ("ext_info", ext_info_value),
        ]);
        let user_coupon_value: serde_json::Value = serde_json::from_str(&user_coupon)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = build_json(&[
            ("openid", serde_json::Value::String(openid.to_string())),
            ("user_coupon", user_coupon_value),
            (
                "recv_time",
                recv_time
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&coupon_url::update_user_coupon_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopCouponServiceImpl.updateUserCouponStatus`：
    /// 构造 `{"openid", "out_user_coupon_id", "out_coupon_id", "status"}` 后
    /// POST `UPDATE_USER_COUPON_STATUS`。
    async fn update_user_coupon_status(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        status: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("openid", serde_json::Value::String(openid.to_string())),
            (
                "out_user_coupon_id",
                serde_json::Value::String(out_user_coupon_id.to_string()),
            ),
            (
                "out_coupon_id",
                serde_json::Value::String(out_coupon_id.to_string()),
            ),
            ("status", serde_json::Value::from(status)),
        ]);
        let response = svc
            .post(
                &coupon_url::update_user_coupon_status_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
