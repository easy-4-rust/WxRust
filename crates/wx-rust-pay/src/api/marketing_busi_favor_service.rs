//! 对应 Java `com.github.binarywang.wxpay.service.MarketingBusiFavorService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// MarketingBusiFavorService（对应 Java `MarketingBusiFavorService`）。
#[async_trait]
pub trait MarketingBusiFavorService: Send + Sync {
    /// 微信支付营销商家券接口
    async fn create_busi_favor_stocks_v3(
        &self,
        request: &BusiFavorStocksCreateRequest,
    ) -> Result<BusiFavorStocksCreateResult, WxErrorException>;

    /// 商家券接口-查询商家券详情API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_2.shtml 接口链接：https:/
    async fn get_busi_favor_stocks_v3(
        &self,
        stock_id: &str,
    ) -> Result<BusiFavorStocksGetResult, WxErrorException>;

    /// 商家券接口-核销用户券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_3.shtml 接口链接：https://a
    async fn verify_busi_favor_coupons_use_v3(
        &self,
        request: &BusiFavorCouponsUseRequest,
    ) -> Result<BusiFavorCouponsUseResult, WxErrorException>;

    /// 商家券接口-H5发券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_4_1.shtml 接口链接：https://ac
    async fn build_busi_favor_couponinfo_url(
        &self,
        request: &BusiFavorCouponsUrlRequest,
    ) -> Result<String, WxErrorException>;

    /// 商家券接口-根据过滤条件查询用户券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_4.shtml 接口链接：htt
    async fn query_busi_favor_users_coupons(
        &self,
        request: &BusiFavorQueryUserCouponsRequest,
    ) -> Result<BusiFavorQueryUserCouponsResult, WxErrorException>;

    /// 商家券接口-查询用户单张券详情API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_5.shtml 接口链接：https
    async fn query_one_busi_favor_users_coupons(
        &self,
        request: &BusiFavorQueryOneUserCouponsRequest,
    ) -> Result<BusiFavorQueryOneUserCouponsResult, WxErrorException>;

    /// 商家券接口-上传预存code API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_6.shtml 接口链接：https
    async fn upload_busi_favor_coupon_codes(
        &self,
        stock_id: &str,
        request: &BusiFavorCouponCodeRequest,
    ) -> Result<BusiFavorCouponCodeResult, WxErrorException>;

    /// 商家券接口-设置商家券事件通知地址 API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_7.shtml 接口链接：ht
    async fn create_busi_favor_callbacks(
        &self,
        request: &BusiFavorCallbacksRequest,
    ) -> Result<BusiFavorCallbacksResult, WxErrorException>;

    /// 商家券接口-查询商家券事件通知地址 API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_8.shtml 接口链接：ht
    async fn query_busi_favor_callbacks(
        &self,
        request: &BusiFavorCallbacksRequest,
    ) -> Result<BusiFavorCallbacksResult, WxErrorException>;

    /// 商家券接口-关联订单信息 API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_9.shtml 接口链接：https:/
    async fn query_busi_favor_coupons_associate(
        &self,
        request: &BusiFavorCouponsAssociateRequest,
    ) -> Result<BusiFavorCouponsAssociateResult, WxErrorException>;

    /// 商家券接口-取消关联订单信息 API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_10.shtml 接口链接：http
    async fn query_busi_favor_coupons_dis_associate(
        &self,
        request: &BusiFavorCouponsAssociateRequest,
    ) -> Result<BusiFavorCouponsAssociateResult, WxErrorException>;

    /// 商家券接口-修改批次预算 API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_11.shtml 接口链接：https:
    async fn update_busi_favor_stocks_budget(
        &self,
        stock_id: &str,
        request: &BusiFavorStocksBudgetRequest,
    ) -> Result<BusiFavorStocksBudgetResult, WxErrorException>;

    /// 商家券接口-创建商家券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_12.shtml 接口链接：https://
    async fn update_busi_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &BusiFavorStocksCreateRequest,
    ) -> Result<String, WxErrorException>;

    /// 商家券接口-申请退款API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_13.shtml 接口链接：https://a
    async fn return_busi_favor_coupons(
        &self,
        request: &BusiFavorCouponsReturnRequest,
    ) -> Result<BusiFavorCouponsReturnResult, WxErrorException>;

    /// 商家券接口-使券失效API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_15.shtml 接口链接：https://a
    async fn deactive_busi_favor_coupons(
        &self,
        request: &BusiFavorCouponsDeactivateRequest,
    ) -> Result<BusiFavorCouponsDeactivateResult, WxErrorException>;

    /// 商家券接口-营销补差付款API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_16.shtml 接口链接：https:/
    async fn subsidy_busi_favor_pay_receipts(
        &self,
        request: &BusiFavorSubsidyRequest,
    ) -> Result<BusiFavorSubsidyResult, WxErrorException>;

    /// 商家券接口-查询营销补差付款单详情API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_17.shtml 接口链接：ht
    async fn query_busi_favor_subsidy_pay_receipts(
        &self,
        subsidy_receipt_id: &str,
    ) -> Result<BusiFavorSubsidyResult, WxErrorException>;

    /// 商家券接口-领券事件回调通知API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter9_2_15.shtml
    async fn notify_busi_favor(
        &self,
        url: &str,
        request: &BusiFavorNotifyRequest,
    ) -> Result<BusiFavorNotifyResult, WxErrorException>;
}
