//! 企业微信待办接口。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpTodoService`。
//!
//! 官方文档：
//! - 获取待办详情：<https://developer.work.weixin.qq.com/document/path/101524>
//! - 更新待办状态：<https://developer.work.weixin.qq.com/document/path/101534>

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::wx_cp_todo::WxCpTodo;
use crate::bean::wx_cp_todo::WxCpTodoAttendee;

/// 企业微信待办接口。
#[async_trait]
pub trait WxCpTodoService: Send + Sync {
    /// 获取待办详情（对应 Java `WxCpTodoService.get(String)`）。
    ///
    /// POST `/cgi-bin/todo/get`，请求参数仅包含必填的 `todo_id`，
    /// 响应直接返回单个待办对象。
    async fn get(&self, todo_id: &str) -> Result<WxCpTodo, WxErrorException>;

    /// 更新待办状态（对应 Java
    /// `WxCpTodoService.update(String, Integer, List)`）。
    ///
    /// POST `/cgi-bin/todo/update`，支持修改待办整体状态（`status` 字段）、
    /// 待办参与人及其状态（`attendees[].userid / status` 字段）。
    /// 仅允许修改当前应用创建的待办，不允许修改已删除的待办。
    ///
    /// - `todo_id`：待办 ID。
    /// - `status`：待办整体状态，可不传：0 - 完成；1 - 进行中。为 `None` 时不修改。
    /// - `attendees`：待办参与人列表，最多支持 20 个参与人。为 `None` 或空时不修改。
    async fn update(
        &self,
        todo_id: &str,
        status: Option<i32>,
        attendees: Option<Vec<WxCpTodoAttendee>>,
    ) -> Result<(), WxErrorException>;
}
