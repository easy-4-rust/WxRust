//! 小程序交易组件-优惠券服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopCouponService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::WxMaShopCouponInfo;
use crate::bean::shop::response::{
    WxMaShopBaseResponse, WxMaShopCouponListResponse, WxMaShopCouponResponse,
    WxMaShopUserCouponListResponse,
};

/// 小程序交易组件-优惠券服务。
#[async_trait]
pub trait WxMaShopCouponService: Send + Sync {
    /// 添加优惠券（对应 Java `addCoupon(WxMaShopCouponInfo)`）。
    async fn add_coupon(
        &self,
        coupon_info: &WxMaShopCouponInfo,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取优惠券（对应 Java `getCoupon(String)`）。
    async fn get_coupon(
        &self,
        out_coupon_id: &str,
    ) -> Result<WxMaShopCouponResponse, WxErrorException>;

    /// 获取优惠券列表（对应 Java `getCouponList(Integer, Integer)`）。
    async fn get_coupon_list(
        &self,
        page_size: Option<i32>,
        offset: Option<i32>,
    ) -> Result<WxMaShopCouponListResponse, WxErrorException>;

    /// 更新优惠券（对应 Java `updateCoupon(WxMaShopCouponInfo)`）。
    async fn update_coupon(
        &self,
        coupon_info: &WxMaShopCouponInfo,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 更新优惠券状态（对应 Java `updateCouponStatus(String, Integer)`）。
    async fn update_coupon_status(
        &self,
        out_coupon_id: &str,
        status: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 更新优惠券库存（对应 Java `updateCouponStock(String, Integer, Integer)`）。
    async fn update_coupon_stock(
        &self,
        out_coupon_id: &str,
        is_used_num: i32,
        receive_num: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 添加用户优惠券（对应 Java `addUserCoupon(String, String, String, Integer, Long)`）。
    async fn add_user_coupon(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        status: i32,
        recv_time: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取用户优惠券列表（对应 Java `getUserCouponList(Integer, Integer, String)`）。
    async fn get_user_coupon_list(
        &self,
        page_size: Option<i32>,
        offset: Option<i32>,
        openid: &str,
    ) -> Result<WxMaShopUserCouponListResponse, WxErrorException>;

    /// 更新用户优惠券（对应 Java `updateUserCoupon(String, String, String, Long, Long)`）。
    async fn update_user_coupon(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        use_time: Option<i64>,
        recv_time: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 更新用户优惠券状态（对应 Java `updateUserCouponStatus(String, String, String, Integer)`）。
    async fn update_user_coupon_status(
        &self,
        openid: &str,
        out_user_coupon_id: &str,
        out_coupon_id: &str,
        status: i32,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
