//! 菜单相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Menu`。

/// 创建菜单。
pub const MENU_CREATE: &str = "/cgi-bin/menu/create?agentid=%d";
/// 删除菜单。
pub const MENU_DELETE: &str = "/cgi-bin/menu/delete?agentid=%d";
/// 获取菜单。
pub const MENU_GET: &str = "/cgi-bin/menu/get?agentid=%d";
