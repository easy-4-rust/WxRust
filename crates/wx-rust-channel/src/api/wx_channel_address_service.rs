//! WxChannelAddressService（对应 Java `me.chanjar.weixin.channel.api.WxChannelAddressService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::address::{
    AddressDetail, AddressIdResponse, AddressInfoResponse, AddressListResponse,
};
use crate::bean::base::WxChannelBaseResponse;

/// 地址管理服务（对应 Java `WxChannelAddressService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_address_service_impl` 的
/// `WxChannelAddressServiceImpl`（Java `WxChannelAddressServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelAddressService: Send + Sync {
    /// 获取地址列表（对应 Java
    /// `WxChannelAddressService#listAddress(Integer, Integer)`）。
    ///
    /// # 参数
    /// - `offset`：起始位置
    /// - `limit`：拉取个数
    async fn list_address(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AddressListResponse, WxErrorException>;

    /// 获取地址详情（对应 Java `WxChannelAddressService#getAddress(String)`）。
    async fn get_address(
        &self,
        address_id: String,
    ) -> Result<AddressInfoResponse, WxErrorException>;

    /// 添加地址（对应 Java `WxChannelAddressService#addAddress(AddressDetail)`）。
    async fn add_address(
        &self,
        address_detail: AddressDetail,
    ) -> Result<AddressIdResponse, WxErrorException>;

    /// 更新地址（对应 Java `WxChannelAddressService#updateAddress(AddressDetail)`）。
    async fn update_address_detail(
        &self,
        address_detail: AddressDetail,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除地址（对应 Java `WxChannelAddressService#deleteAddress(String)`）。
    async fn delete_address(
        &self,
        address_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
