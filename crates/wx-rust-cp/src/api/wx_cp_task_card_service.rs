//! 任务卡片管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpTaskCardService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::TemplateCardMessage;

/// 任务卡片管理服务。
#[async_trait]
pub trait WxCpTaskCardService: Send + Sync {
    /// 更新任务卡片消息状态（对应 Java
    /// `WxCpTaskCardService.update(List<String>, String, String)`；
    /// 使用 `WxCpConfigStorage` 里的 agentId）。
    async fn update(
        &self,
        user_ids: &[&str],
        task_id: &str,
        replace_name: &str,
    ) -> Result<(), WxErrorException>;

    /// 更新按钮为不可点击状态（对应 Java
    /// `WxCpTaskCardService.updateTemplateCardButton(List<String>,
    /// List<Integer>, List<Integer>, Integer, String, String)`）。
    async fn update_template_card_button(
        &self,
        user_ids: &[&str],
        party_ids: &[i32],
        tag_ids: &[i32],
        at_all: i32,
        response_code: &str,
        replace_name: &str,
    ) -> Result<(), WxErrorException>;

    /// 更新任务卡片按钮（对象版，对应 Java
    /// `WxCpTaskCardService.updateTemplateCardButton(TemplateCardMessage)`）。
    async fn update_template_card_button_with_message(
        &self,
        template_card_message: &TemplateCardMessage,
    ) -> Result<(), WxErrorException>;
}
