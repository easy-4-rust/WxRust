//! WxChannelCouponService（对应 Java `me.chanjar.weixin.channel.api.WxChannelCouponService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::coupon::{
    CouponIdResponse, CouponInfoResponse, CouponListParam, CouponListResponse, CouponParam,
    UserCouponListParam, UserCouponListResponse, UserCouponResponse,
};

/// 优惠券服务（对应 Java `WxChannelCouponService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_coupon_service_impl` 的
/// `WxChannelCouponServiceImpl`（Java `WxChannelCouponServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelCouponService: Send + Sync {
    /// 创建优惠券（对应 Java `WxChannelCouponService#createCoupon(CouponParam)`）。
    async fn create_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException>;

    /// 更新优惠券（对应 Java `WxChannelCouponService#updateCoupon(CouponParam)`）。
    async fn update_coupon(
        &self,
        coupon: CouponParam,
    ) -> Result<CouponIdResponse, WxErrorException>;

    /// 更新优惠券状态（对应 Java
    /// `WxChannelCouponService#updateCouponStatus(String, Integer)`）。
    ///
    /// # 参数
    /// - `status`：状态，2 生效、4 已作废、5 删除（`WxCouponStatus`）
    async fn update_coupon_status(
        &self,
        coupon_id: String,
        status: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取优惠券详情（对应 Java `WxChannelCouponService#getCoupon(String)`）。
    async fn get_coupon(&self, coupon_id: String) -> Result<CouponInfoResponse, WxErrorException>;

    /// 获取优惠券 ID 列表（对应 Java
    /// `WxChannelCouponService#getCouponList(CouponListParam)`）。
    async fn get_coupon_list(
        &self,
        param: CouponListParam,
    ) -> Result<CouponListResponse, WxErrorException>;

    /// 获取用户优惠券（对应 Java
    /// `WxChannelCouponService#getUserCoupon(String, String)`）。
    async fn get_user_coupon(
        &self,
        open_id: String,
        user_coupon_id: String,
    ) -> Result<UserCouponResponse, WxErrorException>;

    /// 获取用户优惠券 ID 列表（对应 Java
    /// `WxChannelCouponService#getUserCouponList(UserCouponListParam)`）。
    async fn get_user_coupon_list(
        &self,
        param: UserCouponListParam,
    ) -> Result<UserCouponListResponse, WxErrorException>;
}
