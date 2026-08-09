//! 小程序安全相关接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaSecurityService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaMediaAsyncCheckResult;
use crate::bean::safety::{WxMaUserSafetyRiskRankRequest, WxMaUserSafetyRiskRankResponse};
use crate::bean::security::{
    WxMaMediaSecCheckCheckRequest, WxMaMsgSecCheckCheckRequest, WxMaMsgSecCheckCheckResponse,
};

/// 小程序安全相关接口。
#[async_trait]
pub trait WxMaSecurityService: Send + Sync {
    /// 校验一张图片是否含有违法违规内容（对应 Java `checkImage(File)`）。
    ///
    /// Java 以 `File` 传参；Rust 以文件路径传参（ADAPTED）。
    async fn check_image(&self, file_path: &str) -> Result<bool, WxErrorException>;

    /// 校验一张图片是否含有违法违规内容（对应 Java `checkImage(String fileUrl)`）。
    async fn check_image_url(&self, file_url: &str) -> Result<bool, WxErrorException>;

    /// 检查一段文本是否含有违法违规内容（对应 Java `checkMessage(String)`）。
    async fn check_message(&self, msg_string: &str) -> Result<bool, WxErrorException>;

    /// 检查一段文本是否含有违法违规内容（对应 Java
    /// `checkMessage(WxMaMsgSecCheckCheckRequest)`，新版本接口）。
    async fn check_message_with_request(
        &self,
        msg_request: &WxMaMsgSecCheckCheckRequest,
    ) -> Result<WxMaMsgSecCheckCheckResponse, WxErrorException>;

    /// 异步校验图片/音频是否含有违法违规内容（对应 Java
    /// `mediaCheckAsync(String, int)`）。
    async fn media_check_async(
        &self,
        media_url: &str,
        media_type: i32,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException>;

    /// 异步校验图片/音频是否含有违法违规内容（对应 Java
    /// `mediaCheckAsync(WxMaMediaSecCheckCheckRequest)`，新版本接口）。
    async fn media_check_async_with_request(
        &self,
        request: &WxMaMediaSecCheckCheckRequest,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException>;

    /// 根据提交的用户信息数据获取用户的安全等级（对应 Java
    /// `getUserRiskRank(WxMaUserSafetyRiskRankRequest)`，无需用户授权）。
    async fn get_user_risk_rank(
        &self,
        request: &WxMaUserSafetyRiskRankRequest,
    ) -> Result<WxMaUserSafetyRiskRankResponse, WxErrorException>;
}
