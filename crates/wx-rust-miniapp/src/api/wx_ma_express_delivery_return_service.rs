//! 微信小程序物流退货组件接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaExpressDeliveryReturnService`。
//! 用于处理退货单相关操作，包括新增、查询和取消退货单。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::express::request::WxMaExpressDeliveryReturnAddRequest;
use crate::bean::express::result::WxMaExpressReturnInfoResult;

/// 微信小程序物流退货组件服务。
#[async_trait]
pub trait WxMaExpressDeliveryReturnService: Send + Sync {
    /// 新增退货单（对应 Java `addDeliveryReturn(WxMaExpressDeliveryReturnAddRequest)`）。
    async fn add_delivery_return(
        &self,
        request: &WxMaExpressDeliveryReturnAddRequest,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException>;

    /// 获取退货单信息（对应 Java `getDeliveryReturn(String)`，按 return_id 查询）。
    async fn get_delivery_return(
        &self,
        return_id: &str,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException>;

    /// 取消退货单（对应 Java `unbindDeliveryReturn(String)`，按 return_id 取消）。
    async fn unbind_delivery_return(
        &self,
        return_id: &str,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException>;
}
