//! 企业微信第三方应用标签服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpTagService`：
//! 企业微信第三方开发-标签相关接口。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpTag, WxCpTpTagAddOrRemoveUsersResult, WxCpTpTagGetResult};

/// 企业微信第三方应用标签服务。
#[async_trait]
pub trait WxCpTpTagService: Send + Sync {
    /// 创建标签（对应 Java `create(String, Integer)`：id 非负整型，
    /// 不指定时以目前最大的 id 自增，返回标签 id）。
    async fn create(&self, name: &str, id: Option<i32>) -> Result<String, WxErrorException>;

    /// 更新标签（对应 Java `update(String, String)`）。
    async fn update(&self, tag_id: &str, tag_name: &str) -> Result<(), WxErrorException>;

    /// 删除标签（对应 Java `delete(String)`）。
    async fn delete(&self, tag_id: &str) -> Result<(), WxErrorException>;

    /// 获取标签成员（对应 Java `get(String)`）。
    async fn get(&self, tag_id: &str) -> Result<WxCpTpTagGetResult, WxErrorException>;

    /// 增加标签成员（对应 Java `addUsers2Tag(String, List, List)`）。
    async fn add_users_2_tag(
        &self,
        tag_id: &str,
        user_ids: &[String],
        party_ids: &[String],
    ) -> Result<WxCpTpTagAddOrRemoveUsersResult, WxErrorException>;

    /// 移除标签成员（对应 Java `removeUsersFromTag(String, List, List)`）。
    async fn remove_users_from_tag(
        &self,
        tag_id: &str,
        user_ids: &[String],
        party_ids: &[String],
    ) -> Result<WxCpTpTagAddOrRemoveUsersResult, WxErrorException>;

    /// 获得标签列表（对应 Java `listAll()`）。
    async fn list_all(&self) -> Result<Vec<WxCpTpTag>, WxErrorException>;
}
