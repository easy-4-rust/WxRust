//! 工作台自定义展示服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpAgentWorkBenchService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpAgentWorkBench;

/// 工作台自定义展示服务。
#[async_trait]
pub trait WxCpAgentWorkBenchService: Send + Sync {
    /// 设置工作台模板（对应 Java
    /// `WxCpAgentWorkBenchService.setWorkBenchTemplate(WxCpAgentWorkBench)`）。
    async fn set_work_bench_template(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException>;

    /// 获取工作台模板（对应 Java
    /// `WxCpAgentWorkBenchService.getWorkBenchTemplate(Long)`）。
    async fn get_work_bench_template(&self, agentid: i64) -> Result<String, WxErrorException>;

    /// 设置工作台数据（对应 Java
    /// `WxCpAgentWorkBenchService.setWorkBenchData(WxCpAgentWorkBench)`）。
    async fn set_work_bench_data(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException>;

    /// 批量设置工作台数据（对应 Java
    /// `WxCpAgentWorkBenchService.batchSetWorkBenchData(WxCpAgentWorkBench)`）。
    async fn batch_set_work_bench_data(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException>;
}
