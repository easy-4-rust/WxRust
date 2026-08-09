//! 对应 Java `com.github.binarywang.wxpay.service.MarketingFavorService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// MarketingFavorService（对应 Java `MarketingFavorService`）。
#[async_trait]
pub trait MarketingFavorService: Send + Sync {
    /// 微信支付营销代金券接口
    async fn create_favor_stocks_v3(
        &self,
        request: &FavorStocksCreateRequest,
    ) -> Result<FavorStocksCreateResult, WxErrorException>;

    /// 代金券接口-发放代金券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_2.
    async fn create_favor_coupons_v3(
        &self,
        openid: &str,
        request: &FavorCouponsCreateRequest,
    ) -> Result<FavorCouponsCreateResult, WxErrorException>;

    /// 代金券接口-激活代金券批次API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_
    async fn start_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksStartResult, WxErrorException>;

    /// 代金券接口-条件查询代金券批次列表API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapt
    async fn query_favor_stocks_v3(
        &self,
        request: &FavorStocksQueryRequest,
    ) -> Result<FavorStocksQueryResult, WxErrorException>;

    /// 代金券接口-查询批次详情API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_5
    async fn get_favor_stocks_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
    ) -> Result<FavorStocksGetResult, WxErrorException>;

    /// 代金券接口-查询代金券详情API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_
    async fn get_favor_coupons_v3(
        &self,
        coupon_id: &str,
        appid: &str,
        openid: &str,
    ) -> Result<FavorCouponsGetResult, WxErrorException>;

    /// 代金券接口-查询代金券可用商户API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter
    async fn get_favor_stocks_merchants_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
        offset: i32,
        limit: i32,
    ) -> Result<FavorStocksMerchantsGetResult, WxErrorException>;

    /// 代金券接口-查询代金券可用单品API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter
    async fn get_favor_stocks_items_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
        offset: i32,
        limit: i32,
    ) -> Result<FavorStocksItemsGetResult, WxErrorException>;

    /// 代金券接口-根据商户号查用户的券API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapte
    async fn query_favor_coupons_v3(
        &self,
        request: &FavorCouponsQueryRequest,
    ) -> Result<FavorCouponsQueryResult, WxErrorException>;

    /// 代金券接口-下载批次核销明细API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3
    async fn get_favor_stocks_use_flow_v3(
        &self,
        stock_id: &str,
    ) -> Result<FavorStocksFlowGetResult, WxErrorException>;

    /// 代金券接口-下载批次退款明细API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3
    async fn get_favor_stocks_refund_flow_v3(
        &self,
        stock_id: &str,
    ) -> Result<FavorStocksFlowGetResult, WxErrorException>;

    /// 代金券接口-设置消息通知地址API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3
    async fn save_favor_callbacks_v3(
        &self,
        request: &FavorCallbacksSaveRequest,
    ) -> Result<FavorCallbacksSaveResult, WxErrorException>;

    /// 代金券接口-暂停代金券批次API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_
    async fn pause_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksPauseResult, WxErrorException>;

    /// 代金券接口-重启代金券批次API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/marketing/convention/chapter3_
    async fn restart_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksRestartResult, WxErrorException>;

    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<UseNotifyData, WxErrorException>;

    async fn decrypt_notify_data_resource(
        &self,
        data: &UseNotifyData,
    ) -> Result<FavorCouponsUseResult, WxErrorException>;
}
