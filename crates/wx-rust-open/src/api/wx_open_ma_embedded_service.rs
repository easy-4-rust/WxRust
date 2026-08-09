//! 半屏小程序管理服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaEmbeddedService`，文档：
//! <https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/embedded-management/addEmbedded.html>
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_embedded_*_url`，
//! api_host 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxOpenMaEmbeddedListResult;

/// 半屏小程序管理服务（对应 Java `WxOpenMaEmbeddedService`）。
#[async_trait]
pub trait WxOpenMaEmbeddedService: Send + Sync {
    /// 添加半屏小程序（对应 Java `addEmbedded(String embeddedAppId,
    /// String applyReason)`）。
    async fn add_embedded(
        &self,
        embedded_app_id: &str,
        apply_reason: &str,
    ) -> Result<(), WxErrorException>;

    /// 删除半屏小程序（对应 Java `deleteEmbedded(String embeddedAppId)`；
    /// 删除已经添加到半屏小程序列表的小程序）。
    async fn delete_embedded(&self, embedded_app_id: &str) -> Result<(), WxErrorException>;

    /// 获取半屏小程序调用列表（对应 Java `getEmbeddedList()`，GET 请求）。
    async fn get_embedded_list(&self) -> Result<WxOpenMaEmbeddedListResult, WxErrorException>;

    /// 取消授权小程序（对应 Java
    /// `deleteAuthorizedEmbedded(String embeddedAppId)`）。
    async fn delete_authorized_embedded(
        &self,
        embedded_app_id: &str,
    ) -> Result<(), WxErrorException>;

    /// 获取半屏小程序授权列表，默认分页起始值 0、一次拉取最大值 1000
    /// （对应 Java `getOwnList()`，GET 请求）。
    async fn get_own_list(&self) -> Result<WxOpenMaEmbeddedListResult, WxErrorException>;

    /// 获取半屏小程序授权列表（对应 Java `getOwnList(Integer start,
    /// Integer num)`）。
    ///
    /// `start` 分页起始值默认 0；`num` 一次拉取最大值最大 1000、默认 10
    /// （Java 对 null 取默认值、超 1000 截断为 1000，Rust 以 `Option`
    /// 表达可空入参）。
    async fn get_own_list_with(
        &self,
        start: Option<i32>,
        num: Option<i32>,
    ) -> Result<WxOpenMaEmbeddedListResult, WxErrorException>;

    /// 设置授权方式（对应 Java `setAuthorizedEmbedded(Integer flag)`；
    /// `flag`：0 需要管理员验证，1 自动通过，2 自动拒绝）。
    async fn set_authorized_embedded(&self, flag: i32) -> Result<(), WxErrorException>;
}
