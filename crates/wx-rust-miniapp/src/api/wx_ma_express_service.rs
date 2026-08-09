//! 小程序物流助手接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaExpressService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMaExpressAccount, WxMaExpressAddOrderRequest, WxMaExpressBindAccountRequest,
    WxMaExpressDelivery, WxMaExpressGetOrderRequest, WxMaExpressInfoResult,
    WxMaExpressOrderInfoResult, WxMaExpressPath, WxMaExpressPrinter,
    WxMaExpressPrinterUpdateRequest, WxMaExpressTestUpdateOrderRequest,
};

/// 小程序物流助手接口。
#[async_trait]
pub trait WxMaExpressService: Send + Sync {
    /// 获取支持的快递公司列表（对应 Java `getAllDelivery()`）。
    async fn get_all_delivery(&self) -> Result<Vec<WxMaExpressDelivery>, WxErrorException>;

    /// 获取所有绑定的物流账号（对应 Java `getAllAccount()`）。
    async fn get_all_account(&self) -> Result<Vec<WxMaExpressAccount>, WxErrorException>;

    /// 绑定、解绑物流账号（对应 Java `bindAccount(WxMaExpressBindAccountRequest)`）。
    async fn bind_account(
        &self,
        request: &WxMaExpressBindAccountRequest,
    ) -> Result<WxMaExpressInfoResult, WxErrorException>;

    /// 获取电子面单余额（对应 Java `getQuota(WxMaExpressBindAccountRequest)`，
    /// 仅在使用加盟类快递公司时可以调用）。
    ///
    /// 返回 `quota_num`（Java 返回装箱 `Integer`）。
    async fn get_quota(
        &self,
        request: &WxMaExpressBindAccountRequest,
    ) -> Result<i32, WxErrorException>;

    /// 配置面单打印员（对应 Java `updatePrinter(WxMaExpressPrinterUpdateRequest)`）。
    async fn update_printer(
        &self,
        request: &WxMaExpressPrinterUpdateRequest,
    ) -> Result<(), WxErrorException>;

    /// 获取打印员（对应 Java `getPrinter()`）。
    async fn get_printer(&self) -> Result<WxMaExpressPrinter, WxErrorException>;

    /// 生成运单（对应 Java `addOrder(WxMaExpressAddOrderRequest)`）。
    async fn add_order(
        &self,
        request: &WxMaExpressAddOrderRequest,
    ) -> Result<WxMaExpressOrderInfoResult, WxErrorException>;

    /// 批量获取运单数据（对应 Java `batchGetOrder(List<WxMaExpressGetOrderRequest>)`，
    /// 最多不能超过 1000 个）。
    async fn batch_get_order(
        &self,
        requests: &[WxMaExpressGetOrderRequest],
    ) -> Result<Vec<WxMaExpressOrderInfoResult>, WxErrorException>;

    /// 取消运单（对应 Java `cancelOrder(WxMaExpressGetOrderRequest)`）。
    async fn cancel_order(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<(), WxErrorException>;

    /// 获取运单数据（对应 Java `getOrder(WxMaExpressGetOrderRequest)`）。
    async fn get_order(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<WxMaExpressOrderInfoResult, WxErrorException>;

    /// 查询运单轨迹（对应 Java `getPath(WxMaExpressGetOrderRequest)`）。
    async fn get_path(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<WxMaExpressPath, WxErrorException>;

    /// 模拟快递公司更新订单状态（对应 Java
    /// `testUpdateOrder(WxMaExpressTestUpdateOrderRequest)`，该接口只能用户测试）。
    async fn test_update_order(
        &self,
        request: &WxMaExpressTestUpdateOrderRequest,
    ) -> Result<(), WxErrorException>;
}
