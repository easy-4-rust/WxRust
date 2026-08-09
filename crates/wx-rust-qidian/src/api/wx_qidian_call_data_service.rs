//! 通话数据服务接口。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.WxQidianCallDataService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::call::GetSwitchBoardListResponse;

/// 通话数据相关操作接口。
#[async_trait]
pub trait WxQidianCallDataService: Send + Sync {
    /// 拉取总机号列表（对应 Java `getSwitchBoardList()`）。
    async fn get_switch_board_list(&self) -> Result<GetSwitchBoardListResponse, WxErrorException>;
}
