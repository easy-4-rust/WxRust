//! WxChannelBasicService（对应 Java `me.chanjar.weixin.channel.api.WxChannelBasicService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::address::AddressCodeResponse;
use crate::bean::image::{ChannelImageInfo, ChannelImageResponse, QualificationFileResponse};
use crate::bean::shop::{
    ShopH5UrlResponse, ShopInfoResponse, ShopQrCodeResponse, ShopTagLinkResponse,
};

/// 基础接口服务（对应 Java `WxChannelBasicService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_basic_service_impl` 的
/// `WxChannelBasicServiceImpl`（Java `WxChannelBasicServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelBasicService: Send + Sync {
    /// 获取店铺基本信息（对应 Java `WxChannelBasicService#getShopInfo`）。
    async fn get_shop_info(&self) -> Result<ShopInfoResponse, WxErrorException>;

    /// 上传图片（对应 Java `WxChannelBasicService#uploadImg(int, String)`）。
    ///
    /// # 参数
    /// - `resp_type`：0 返回 media_id 和 pay_media_id；1 返回图片链接
    ///   （商品信息相关图片请务必使用此参数得到链接）
    /// - `img_url`：图片 url
    async fn upload_img(
        &self,
        resp_type: i32,
        img_url: String,
    ) -> Result<ChannelImageInfo, WxErrorException>;

    /// 上传图片（对应 Java `WxChannelBasicService#uploadImg(int, File, int, int)`）。
    ///
    /// # 参数
    /// - `resp_type`：0 返回 media_id 和 pay_media_id；1 返回图片链接
    /// - `file`：图片文件
    /// - `height`：图片的高，单位：像素
    /// - `width`：图片的宽，单位：像素
    async fn upload_img_with_file(
        &self,
        resp_type: i32,
        file: std::path::PathBuf,
        height: i32,
        width: i32,
    ) -> Result<ChannelImageInfo, WxErrorException>;

    /// 上传资质图片（对应 Java `WxChannelBasicService#uploadQualificationFile(File)`）。
    async fn upload_qualification_file(
        &self,
        file: std::path::PathBuf,
    ) -> Result<QualificationFileResponse, WxErrorException>;

    /// 根据 media_id 获取图片（对应 Java `WxChannelBasicService#getImg(String)`）。
    async fn get_img(&self, media_id: String) -> Result<ChannelImageResponse, WxErrorException>;

    /// 获取地址编码（最多获取 4 级；对应 Java
    /// `WxChannelBasicService#getAddressCode(Integer)`）。
    ///
    /// # 参数
    /// - `code`：地址行政编码，不填或者填 0 时，拉取全国的省级行政编码
    async fn get_address_code(
        &self,
        code: Option<i32>,
    ) -> Result<AddressCodeResponse, WxErrorException>;

    /// 获取店铺 H5 链接（对应 Java `WxChannelBasicService#getShopH5Url`）。
    async fn get_shop_h5_url(&self) -> Result<ShopH5UrlResponse, WxErrorException>;

    /// 获取店铺二维码（对应 Java `WxChannelBasicService#getShopQrCode(int)`）。
    ///
    /// # 参数
    /// - `qrcode_type`：二维码类型
    async fn get_shop_qr_code(
        &self,
        qrcode_type: i32,
    ) -> Result<ShopQrCodeResponse, WxErrorException>;

    /// 获取店铺口令（对应 Java `WxChannelBasicService#getShopTagLink`）。
    async fn get_shop_tag_link(&self) -> Result<ShopTagLinkResponse, WxErrorException>;
}
