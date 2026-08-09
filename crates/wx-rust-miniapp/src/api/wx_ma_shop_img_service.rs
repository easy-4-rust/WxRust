//! 小程序交易组件-接入商品前必需接口（图片上传）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopImgService`。

use async_trait::async_trait;
use wx_rust_common::bean::result::WxMinishopImageUploadCustomizeResult;
use wx_rust_common::error::WxErrorException;

/// 小程序交易组件-图片上传服务。
#[async_trait]
pub trait WxMaShopImgService: Send + Sync {
    /// 上传图片（对应 Java `uploadImg(File)`，respType 固定 "0"）。
    async fn upload_img(
        &self,
        file_path: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException>;

    /// 上传图片，带 respType 参数（对应 Java `uploadImg(File, String)`）。
    async fn upload_img_with_resp_type(
        &self,
        file_path: &str,
        resp_type: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException>;

    /// 上传图片链接，带 respType 参数（对应 Java `uploadImg(String, String)`，
    /// 以 multipart 表单携带 img_url，upload_type=1）。
    async fn upload_img_from_url(
        &self,
        img_url: &str,
        resp_type: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException>;
}
