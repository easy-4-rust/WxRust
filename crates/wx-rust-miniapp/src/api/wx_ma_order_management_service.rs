//! 小程序订单管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaOrderManagementService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::order::{WxMaOrderManagementGetOrderDetailPath, WxMaOrderManagementResult};

/// 小程序订单管理服务。
#[async_trait]
pub trait WxMaOrderManagementService: Send + Sync {
    /// 查询订单详情路径（对应 Java `getOrderDetailPath()`）。
    ///
    /// 如果没有配置过订单详情路径，会返回成功，其中 path 为 ''。
    async fn get_order_detail_path(
        &self,
    ) -> Result<WxMaOrderManagementGetOrderDetailPath, WxErrorException>;

    /// 配置订单详情路径（对应 Java `updateOrderDetailPath(String)`）。
    ///
    /// path 必须包含字符串 "${商品订单号}"。
    async fn update_order_detail_path(
        &self,
        path: &str,
    ) -> Result<WxMaOrderManagementResult, WxErrorException>;
}
