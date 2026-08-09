//! 企业微信第三方应用通讯录服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpContactService`：
//! 通讯录单个搜索（https://work.weixin.qq.com/api/doc/90001/90143/91844）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpContactSearch, WxCpTpContactSearchResp};

/// 企业微信第三方应用通讯录服务。
#[async_trait]
pub trait WxCpTpContactService: Send + Sync {
    /// 通讯录单个搜索（对应 Java `contactSearch(WxCpTpContactSearch)`）。
    async fn contact_search(
        &self,
        search: &WxCpTpContactSearch,
    ) -> Result<WxCpTpContactSearchResp, WxErrorException>;
}
