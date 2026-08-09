//! 小程序交易组件-物流发货服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopDeliveryService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::{WxMaShopDeliveryRecieveRequest, WxMaShopDeliverySendRequest};
use crate::bean::shop::response::{WxMaShopBaseResponse, WxMaShopDeliveryGetCompanyListResponse};

/// 小程序交易组件-物流发货服务。
#[async_trait]
pub trait WxMaShopDeliveryService: Send + Sync {
    /// 获取快递公司列表（对应 Java `getCompanyList()`）。
    async fn get_company_list(
        &self,
    ) -> Result<WxMaShopDeliveryGetCompanyListResponse, WxErrorException>;

    /// 订单发货（对应 Java `send(WxMaShopDeliverySendRequest)`）。
    async fn send(
        &self,
        request: &WxMaShopDeliverySendRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 订单确认收货（对应 Java `receive(WxMaShopDeliveryRecieveRequest)`）。
    async fn receive(
        &self,
        request: &WxMaShopDeliveryRecieveRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
