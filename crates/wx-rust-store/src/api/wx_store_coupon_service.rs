//! WxStoreCouponService（对应 Java `com.binarywang.wxjava.store.api.WxStoreCouponService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::coupon::{
    CouponIdResponse, CouponInfoResponse, CouponListParam, CouponListResponse, CouponParam,
    UserCouponListParam, UserCouponListResponse, UserCouponResponse,
};

/// 优惠券服务（对应 Java `WxStoreCouponService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_coupon_service_impl` 的
/// `WxStoreCouponServiceImpl`（Java `WxStoreCouponServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreCouponService: Send + Sync {
    /// 创建优惠券（对应 Java `WxStoreCouponService#createCoupon(CouponParam)`）。
    async fn create_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException>;

    /// 更新优惠券（对应 Java `WxStoreCouponService#updateCoupon(CouponParam)`）。
    async fn update_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException>;

    /// 更新优惠券状态（对应 Java
    /// `WxStoreCouponService#updateCouponStatus(String, Integer)`）。
    ///
    /// # 参数
    /// - `status`：状态，2 生效、4 已作废、5 删除（`WxCouponStatus`）
    async fn update_coupon_status(
        &self,
        coupon_id: String,
        status: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取优惠券详情（对应 Java `WxStoreCouponService#getCoupon(String)`）。
    async fn get_coupon(&self, coupon_id: String) -> Result<CouponInfoResponse, WxErrorException>;

    /// 获取优惠券 ID 列表（对应 Java
    /// `WxStoreCouponService#getCouponList(CouponListParam)`）。
    async fn get_coupon_list(
        &self,
        param: CouponListParam,
    ) -> Result<CouponListResponse, WxErrorException>;

    /// 获取用户优惠券（对应 Java
    /// `WxStoreCouponService#getUserCoupon(String, String)`）。
    async fn get_user_coupon(
        &self,
        open_id: String,
        user_coupon_id: String,
    ) -> Result<UserCouponResponse, WxErrorException>;

    /// 获取用户优惠券 ID 列表（对应 Java
    /// `WxStoreCouponService#getUserCouponList(UserCouponListParam)`）。
    async fn get_user_coupon_list(
        &self,
        param: UserCouponListParam,
    ) -> Result<UserCouponListResponse, WxErrorException>;
}
