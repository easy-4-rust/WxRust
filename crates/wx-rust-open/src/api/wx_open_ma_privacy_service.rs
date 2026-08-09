//! 小程序用户隐私保护指引服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaPrivacyService`。
//! 从 2022 年 4 月 18 日开始，部分小程序前端 api 需申请后，方可使用。
//! 该接口用于获取「需申请并审核通过」后才可使用的接口列表。
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_privacy_*_url`，
//! api_host 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::ApplyPrivacyInterface;
use crate::bean::ApplyPrivacyInterfaceResult;
use crate::bean::GetPrivacyInterfaceResult;
use crate::bean::GetPrivacySettingResult;
use crate::bean::SetPrivacySetting;
use crate::bean::UploadPrivacyFileResult;

/// 微信第三方平台 小程序用户隐私保护指引服务（对应 Java
/// `WxOpenMaPrivacyService`）。
#[async_trait]
pub trait WxOpenMaPrivacyService: Send + Sync {
    /// 查询小程序用户隐私保护指引（对应 Java
    /// `getPrivacySetting(Integer privacyVer)`）。
    ///
    /// `privacy_ver`：1 表示现网版本，2 表示开发版，默认是 2；Java
    /// 可空（null 时不携带该字段），Rust 以 `Option` 表达。
    async fn get_privacy_setting(
        &self,
        privacy_ver: Option<i32>,
    ) -> Result<GetPrivacySettingResult, WxErrorException>;

    /// 设置小程序用户隐私保护指引（对应 Java
    /// `setPrivacySetting(SetPrivacySetting dto)`）。
    async fn set_privacy_setting(&self, dto: &SetPrivacySetting) -> Result<(), WxErrorException>;

    /// 上传小程序用户隐私保护指引文件（对应 Java
    /// `uploadPrivacyFile(String content)`）。
    ///
    /// 仅限文本文件，限制文件大小不超过 100kb，否则会报错。
    /// NOT_MIRRORED：Java 实现本身未完成（上游 TODO，恒抛
    /// `WxError(5003, "暂未实现用户隐私指引内容上传")`），Rust 严格镜像
    /// 该错误（`Err(5003)`）。
    async fn upload_privacy_file(
        &self,
        _content: &str,
    ) -> Result<UploadPrivacyFileResult, WxErrorException> {
        Err(WxErrorException::from_code(
            5003,
            "暂未实现用户隐私指引内容上传",
        ))
    }

    /// 隐私接口-获取接口列表（对应 Java `getPrivacyInterface()`，
    /// GET 请求）。
    async fn get_privacy_interface(&self) -> Result<GetPrivacyInterfaceResult, WxErrorException>;

    /// 隐私接口-申请接口（对应 Java
    /// `applyPrivacyInterface(ApplyPrivacyInterface dto)`）。
    async fn apply_privacy_interface(
        &self,
        dto: &ApplyPrivacyInterface,
    ) -> Result<ApplyPrivacyInterfaceResult, WxErrorException>;
}
