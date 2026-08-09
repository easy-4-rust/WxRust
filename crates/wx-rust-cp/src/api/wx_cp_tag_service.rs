//! 标签管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpTagService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTag, WxCpTagAddOrRemoveUsersResult, WxCpTagGetResult, WxCpUser};

/// 标签管理服务。
#[async_trait]
pub trait WxCpTagService: Send + Sync {
    /// 创建标签（对应 Java `WxCpTagService.create(String, Integer)`；
    /// `id` 不指定时以目前最大的 id 自增）。
    async fn create(&self, name: &str, id: Option<i32>) -> Result<String, WxErrorException>;

    /// 更新标签（对应 Java `WxCpTagService.update(String, String)`）。
    async fn update(&self, tag_id: &str, tag_name: &str) -> Result<(), WxErrorException>;

    /// 删除标签（对应 Java `WxCpTagService.delete(String)`）。
    async fn delete(&self, tag_id: &str) -> Result<(), WxErrorException>;

    /// 获取标签列表（对应 Java `WxCpTagService.listAll()`）。
    async fn list_all(&self) -> Result<Vec<WxCpTag>, WxErrorException>;

    /// 获取标签成员（对应 Java `WxCpTagService.listUsersByTagId(String)`，
    /// 返回 `userlist` 数组）。
    async fn list_users_by_tag_id(&self, tag_id: &str) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 获取标签详情（对应 Java `WxCpTagService.get(String)`；Java 对
    /// `tagId` 为 null 抛 `IllegalArgumentException`，Rust `&str` 无 null，
    /// 免检，ADAPTED）。
    async fn get(&self, tag_id: &str) -> Result<WxCpTagGetResult, WxErrorException>;

    /// 增加标签成员（对应 Java
    /// `WxCpTagService.addUsers2Tag(String, List<String>, List<String>)`）。
    async fn add_users2_tag(
        &self,
        tag_id: &str,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpTagAddOrRemoveUsersResult, WxErrorException>;

    /// 移除标签成员（对应 Java
    /// `WxCpTagService.removeUsersFromTag(String, List<String>,
    /// List<String>)`）。
    async fn remove_users_from_tag(
        &self,
        tag_id: &str,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpTagAddOrRemoveUsersResult, WxErrorException>;
}
