//! 应用/工作台相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Agent` 与 `WxCpApiPathConsts.WorkBench`。

/// 应用信息相关接口（对应 Java `WxCpApiPathConsts.Agent`）。
pub mod agent {
    /// 获取应用信息。
    pub const AGENT_GET: &str = "/cgi-bin/agent/get?agentid=%d";
    /// 设置应用信息。
    pub const AGENT_SET: &str = "/cgi-bin/agent/set";
    /// 获取应用列表。
    pub const AGENT_LIST: &str = "/cgi-bin/agent/list";
    /// 获取应用管理员列表。
    pub const AGENT_GET_ADMIN_LIST: &str = "/cgi-bin/agent/get_admin_list";
}

/// 工作台相关接口（对应 Java `WxCpApiPathConsts.WorkBench`）。
pub mod work_bench {
    /// 设置工作台自定义展示。
    pub const WORKBENCH_TEMPLATE_SET: &str = "/cgi-bin/agent/set_workbench_template";
    /// 获取工作台自定义展示。
    pub const WORKBENCH_TEMPLATE_GET: &str = "/cgi-bin/agent/get_workbench_template";
    /// 设置工作台展示数据。
    pub const WORKBENCH_DATA_SET: &str = "/cgi-bin/agent/set_workbench_data";
    /// 设置工作台展示数据（批量）。
    pub const WORKBENCH_BATCH_DATA_SET: &str = "/cgi-bin/agent/batch_set_workbench_data";
}
