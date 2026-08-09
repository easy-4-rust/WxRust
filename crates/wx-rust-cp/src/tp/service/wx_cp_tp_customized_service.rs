//! 企业微信第三方应用代开发服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpCustomizedService`：
//! 获取应用模板列表/代开发应用详情
//! （https://developer.work.weixin.qq.com/document/path/97111）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpCustomizedAppDetail, WxCpTpTemplateList};

/// 企业微信第三方应用代开发服务。
#[async_trait]
pub trait WxCpTpCustomizedService: Send + Sync {
    /// 获取应用模板列表（对应 Java `getTemplateList()`）。
    async fn get_template_list(&self) -> Result<WxCpTpTemplateList, WxErrorException>;

    /// 获取代开发应用详情（对应 Java
    /// `getCustomizedAppDetail(String, Integer)`：agentId 为空时返回该
    /// 企业所有的代开发自建应用详情）。
    async fn get_customized_app_detail(
        &self,
        auth_corp_id: &str,
        agent_id: Option<i32>,
    ) -> Result<WxCpTpCustomizedAppDetail, WxErrorException>;
}
