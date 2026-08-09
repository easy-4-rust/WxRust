//! 订阅消息服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaSubscribeService`。

use async_trait::async_trait;
use wx_rust_common::bean::subscribemsg::{
    CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo,
};
use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMaGetUserNotifyRequest, WxMaGetUserNotifyResult, WxMaServiceNotifyExtRequest,
    WxMaServiceNotifyRequest, WxMaSubscribeMessage,
};

/// 订阅消息服务。
#[async_trait]
pub trait WxMaSubscribeService: Send + Sync {
    /// 获取账号所属类目下的公共模板标题（对应 Java
    /// `WxMaSubscribeService.getPubTemplateTitleList(String[], int, int)`）。
    ///
    /// GET `/wxaapi/newtmpl/getpubtemplatetitles?ids=&start=&limit=`（ids 以
    /// `,` 连接）。
    async fn get_pub_template_title_list(
        &self,
        ids: &[&str],
        start: i32,
        limit: i32,
    ) -> Result<PubTemplateTitleListResult, WxErrorException>;

    /// 获取模板库某个模板标题下关键词库（对应 Java
    /// `WxMaSubscribeService.getPubTemplateKeyWordsById(String)`）。
    ///
    /// GET `/wxaapi/newtmpl/getpubtemplatekeywords?tid=...`，响应取 `data` 数组。
    async fn get_pub_template_keywords_by_id(
        &self,
        id: &str,
    ) -> Result<Vec<PubTemplateKeyword>, WxErrorException>;

    /// 组合模板并添加至账号下的个人模板库（对应 Java
    /// `WxMaSubscribeService.addTemplate(String, List<Integer>, String)`）。
    ///
    /// POST `/wxaapi/newtmpl/addtemplate`，请求体 `tid`/`kidList`/`sceneDesc`；
    /// 返回添加至账号下的模板 id（`priTmplId`）。
    async fn add_template(
        &self,
        id: &str,
        keyword_id_list: &[i32],
        scene_desc: &str,
    ) -> Result<String, WxErrorException>;

    /// 获取当前账号下的个人模板列表（对应 Java
    /// `WxMaSubscribeService.getTemplateList()`）。
    ///
    /// GET `/wxaapi/newtmpl/gettemplate`，响应取 `data` 数组。
    async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException>;

    /// 删除账号下的某个模板（对应 Java
    /// `WxMaSubscribeService.delTemplate(String)`，成功返回 true）。
    async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException>;

    /// 获取小程序账号的类目（对应 Java `WxMaSubscribeService.getCategory()`）。
    ///
    /// GET `/wxaapi/newtmpl/getcategory`，响应取 `data` 数组。
    async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException>;

    /// 发送订阅消息（对应 Java
    /// `WxMaSubscribeService.sendSubscribeMsg(WxMaSubscribeMessage)`）。
    ///
    /// POST `/cgi-bin/message/subscribe/send`，请求体走
    /// `WxMaSubscribeMessage` 的手写 Serialize（`touser`/`template_id`/
    /// `page`/`miniprogram_state`/`lang`/`data` map 结构）。
    async fn send_subscribe_msg(
        &self,
        subscribe_message: &WxMaSubscribeMessage,
    ) -> Result<(), WxErrorException>;

    /// 激活与更新服务卡片（对应 Java
    /// `WxMaSubscribeService.setUserNotify(WxMaServiceNotifyRequest)`）。
    async fn set_user_notify(
        &self,
        request: &WxMaServiceNotifyRequest,
    ) -> Result<(), WxErrorException>;

    /// 更新服务卡片扩展信息（对应 Java
    /// `WxMaSubscribeService.setUserNotifyExt(WxMaServiceNotifyExtRequest)`）。
    async fn set_user_notify_ext(
        &self,
        request: &WxMaServiceNotifyExtRequest,
    ) -> Result<(), WxErrorException>;

    /// 查询服务卡片状态（对应 Java
    /// `WxMaSubscribeService.getUserNotify(WxMaGetUserNotifyRequest)`）。
    async fn get_user_notify(
        &self,
        request: &WxMaGetUserNotifyRequest,
    ) -> Result<WxMaGetUserNotifyResult, WxErrorException>;
}
