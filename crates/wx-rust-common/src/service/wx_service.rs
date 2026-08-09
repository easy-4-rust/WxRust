//! 微信服务接口。
//!
//! 对应 Java `me.chanjar.weixin.common.service.WxService`。

use async_trait::async_trait;
use serde::Serialize;

use crate::bean::CommonUploadParam;
use crate::error::WxErrorException;

/// 微信服务接口。
///
/// 所有业务模块的顶层 Service（`WxMpService`/`WxMaService` 等）均扩展此接口。
/// 当某个 API 未在 Service 中实现时，可用这些通用方法直接调用微信接口。
#[async_trait]
pub trait WxService: Send + Sync {
    /// 针对所有微信 API 中的 GET 请求。
    ///
    /// # 参数
    /// - `url`：请求接口地址
    /// - `query_param`：查询参数串（如 `a=1&b=2`）
    ///
    /// # 返回
    /// 接口响应字符串
    async fn get(&self, url: &str, query_param: Option<&str>) -> Result<String, WxErrorException>;

    /// 针对所有微信 API 中的 POST 请求（请求体为字符串）。
    ///
    /// # 参数
    /// - `url`：请求接口地址
    /// - `post_data`：请求参数（json 值或 form 串）
    ///
    /// # 返回
    /// 接口响应字符串
    async fn post(&self, url: &str, post_data: &str) -> Result<String, WxErrorException>;

    /// 针对所有微信 API 中的 POST 请求（请求体为可序列化对象）。
    ///
    /// # 参数
    /// - `url`：请求接口地址
    /// - `obj`：请求对象（自动序列化为 JSON）
    ///
    /// # 返回
    /// 接口响应字符串
    async fn post_json<T: Serialize + Send + Sync>(
        &self,
        url: &str,
        obj: &T,
    ) -> Result<String, WxErrorException>;

    /// 针对所有微信 API 中的 POST 文件上传请求。
    ///
    /// # 参数
    /// - `url`：请求接口地址
    /// - `param`：文件上传参数
    ///
    /// # 返回
    /// 接口响应字符串
    async fn upload(&self, url: &str, param: CommonUploadParam)
    -> Result<String, WxErrorException>;
}
