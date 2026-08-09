//! 通话数据服务实现。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.impl.WxQidianCallDataServiceImpl`：
//! 通过门面 `get` 调用总机号列表接口
//! （`https://api.qidian.qq.com/cgi-bin/call/callData/getswitchboardlist`，
//! 经执行引擎自动注入 access_token）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxQidianCallDataService, WxQidianService};
use crate::bean::call::GetSwitchBoardListResponse;
use crate::enums::wx_qidian_api_url::call_data::GET_SWITCH_BOARD_LIST;

/// 通话数据服务实现。
pub struct WxQidianCallDataServiceImpl {
    /// 门面服务弱引用（对应 Java `WxQidianService wxQidianService` 字段）
    service: Weak<dyn WxQidianService>,
}

impl WxQidianCallDataServiceImpl {
    /// 构建实现。
    ///
    /// # 参数
    /// - `service`：门面服务弱引用（打破循环引用）
    pub fn new(service: Weak<dyn WxQidianService>) -> Self {
        Self { service }
    }

    /// 门面服务引用（子服务生命周期内必然存在，对应 Java 强引用字段）。
    fn service(&self) -> Result<std::sync::Arc<dyn WxQidianService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已释放"))
    }
}

#[async_trait]
impl WxQidianCallDataService for WxQidianCallDataServiceImpl {
    async fn get_switch_board_list(&self) -> Result<GetSwitchBoardListResponse, WxErrorException> {
        let service = self.service()?;
        let result = service.get(&GET_SWITCH_BOARD_LIST, "").await?;
        GetSwitchBoardListResponse::from_json(&result).map_err(WxErrorException::Serde)
    }
}
