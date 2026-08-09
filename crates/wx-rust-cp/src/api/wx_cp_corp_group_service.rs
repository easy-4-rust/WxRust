//! 企业互联服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpCorpGroupService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpCorpGroupCorp;

/// 企业互联服务。
#[async_trait]
pub trait WxCpCorpGroupService: Send + Sync {
    /// 获取应用共享信息列表（对应 Java
    /// `WxCpCorpGroupService.listAppShareInfo(Integer, Integer, String,
    /// Integer, String)`）。
    async fn list_app_share_info(
        &self,
        agent_id: Option<i32>,
        business_type: Option<i32>,
        corp_id: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<Vec<WxCpCorpGroupCorp>, WxErrorException>;
}
