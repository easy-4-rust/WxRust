//! WxStoreAddressService（对应 Java `com.binarywang.wxjava.store.api.WxStoreAddressService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::address::{
    AddressDetail, AddressIdResponse, AddressInfoResponse, AddressListResponse,
};
use crate::bean::base::WxStoreBaseResponse;

/// 地址管理服务（对应 Java `WxStoreAddressService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_address_service_impl` 的
/// `WxStoreAddressServiceImpl`（Java `WxStoreAddressServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreAddressService: Send + Sync {
    /// 获取地址列表（对应 Java
    /// `WxStoreAddressService#listAddress(Integer, Integer)`）。
    ///
    /// # 参数
    /// - `offset`：起始位置
    /// - `limit`：拉取个数
    async fn list_address(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<AddressListResponse, WxErrorException>;

    /// 获取地址详情（对应 Java `WxStoreAddressService#getAddress(String)`）。
    async fn get_address(
        &self,
        address_id: String,
    ) -> Result<AddressInfoResponse, WxErrorException>;

    /// 添加地址（对应 Java `WxStoreAddressService#addAddress(AddressDetail)`）。
    async fn add_address(
        &self,
        address_detail: AddressDetail,
    ) -> Result<AddressIdResponse, WxErrorException>;

    /// 更新地址（对应 Java `WxStoreAddressService#updateAddress(AddressDetail)`）。
    async fn update_address_detail(
        &self,
        address_detail: AddressDetail,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 删除地址（对应 Java `WxStoreAddressService#deleteAddress(String)`）。
    async fn delete_address(
        &self,
        address_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
