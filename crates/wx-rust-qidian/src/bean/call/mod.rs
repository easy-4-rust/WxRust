//! 通话数据对象。
//!
//! 对应 Java `me.chanjar.weixin.qidian.bean.call` 包。

pub mod get_switch_board_list_response;
pub mod switch_board;
pub mod switch_board_list;

pub use get_switch_board_list_response::GetSwitchBoardListResponse;
pub use switch_board::SwitchBoard;
pub use switch_board_list::SwitchBoardList;
