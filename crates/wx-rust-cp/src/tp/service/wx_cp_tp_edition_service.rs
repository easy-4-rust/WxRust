//! 企业微信第三方应用应用版本付费版本服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpEditionService`：
//! 延长试用期（https://developer.work.weixin.qq.com/document/path/91913；
//! 一个应用可多次延长试用，试用总天数不能超过 60 天）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpTpProlongTryResult;

/// 企业微信第三方应用应用版本付费版本服务。
#[async_trait]
pub trait WxCpTpEditionService: Send + Sync {
    /// 延长试用期（对应 Java `prolongTry(String, Integer, String)`：
    /// buyerCorpId 购买方 corpId，prolongDays 延长天数，appId 仅旧套件
    /// 需要填）。
    async fn prolong_try(
        &self,
        buyer_corp_id: &str,
        prolong_days: Option<i32>,
        app_id: &str,
    ) -> Result<WxCpTpProlongTryResult, WxErrorException>;
}
