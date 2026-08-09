//! 标签相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Tag`。

/// 创建标签。
pub const TAG_CREATE: &str = "/cgi-bin/tag/create";
/// 更新标签名字。
pub const TAG_UPDATE: &str = "/cgi-bin/tag/update";
/// 删除标签（`tagid` 拼在路径后）。
pub const TAG_DELETE: &str = "/cgi-bin/tag/delete?tagid=%s";
/// 获取标签列表。
pub const TAG_LIST: &str = "/cgi-bin/tag/list";
/// 获取标签成员（`tagid` 拼在路径后）。
pub const TAG_GET: &str = "/cgi-bin/tag/get?tagid=%s";
/// 增加标签成员。
pub const TAG_ADD_TAG_USERS: &str = "/cgi-bin/tag/addtagusers";
/// 删除标签成员。
pub const TAG_DEL_TAG_USERS: &str = "/cgi-bin/tag/deltagusers";
