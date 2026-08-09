//! 工作台自定义展示服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpAgentWorkBenchServiceImpl`。
//! Java `WxCpAgentWorkBench` 的 `toTemplateString`/`toUserDataString`/
//! `toBatchUserDataString` 序列化逻辑在 bean 中未落地，按 Java 类内
//! 实现镜像到本文件（键序与条件一致）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpAgentWorkBenchService, WxCpService};
use crate::bean::WxCpAgentWorkBench;
use crate::enums::url_agent::work_bench::*;

/// 工作台自定义展示服务实现。
pub struct WxCpAgentWorkBenchServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpAgentWorkBenchServiceImpl {
    /// 构建工作台服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpAgentWorkBenchService for WxCpAgentWorkBenchServiceImpl {
    async fn set_work_bench_template(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `setWorkBenchTemplate`：POST `WORKBENCH_TEMPLATE_SET`，
        // 请求体 `toTemplateString()`
        let config = svc.wx_cp_config_storage();
        let body = to_template_string(wx_cp_agent_work_bench);
        svc.post(&config.api_url(WORKBENCH_TEMPLATE_SET), &body)
            .await?;
        Ok(())
    }

    async fn get_work_bench_template(&self, agent_id: i64) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getWorkBenchTemplate`：POST `WORKBENCH_TEMPLATE_GET`
        // `{"agentid":...}`，直接返回响应原文
        let body = serde_json::json!({ "agentid": agent_id }).to_string();
        let config = svc.wx_cp_config_storage();
        svc.post(&config.api_url(WORKBENCH_TEMPLATE_GET), &body)
            .await
    }

    async fn set_work_bench_data(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `setWorkBenchData`：POST `WORKBENCH_DATA_SET`，
        // 请求体 `toUserDataString()`
        let config = svc.wx_cp_config_storage();
        let body = to_user_data_string(wx_cp_agent_work_bench);
        svc.post(&config.api_url(WORKBENCH_DATA_SET), &body).await?;
        Ok(())
    }

    async fn batch_set_work_bench_data(
        &self,
        wx_cp_agent_work_bench: &WxCpAgentWorkBench,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchSetWorkBenchData`：POST `WORKBENCH_BATCH_DATA_SET`，
        // 请求体 `toBatchUserDataString()`
        let config = svc.wx_cp_config_storage();
        let body = to_batch_user_data_string(wx_cp_agent_work_bench);
        svc.post(&config.api_url(WORKBENCH_BATCH_DATA_SET), &body)
            .await?;
        Ok(())
    }
}

/// 生成模板 Json 字符串（对应 Java `WxCpAgentWorkBench.toTemplateString()`：
/// `agentid`/`type`/`replace_user_data`(非 null) + `handle`）。
fn to_template_string(w: &WxCpAgentWorkBench) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("agentid".to_string(), serde_json::json!(w.agent_id));
    obj.insert(
        "type".to_string(),
        serde_json::Value::from(w.r#type.as_str()),
    );
    if w.replace_user_data {
        obj.insert(
            "replace_user_data".to_string(),
            serde_json::Value::from(w.replace_user_data),
        );
    }
    handle(w, &mut obj, false);
    serde_json::Value::Object(obj).to_string()
}

/// 生成用户数据 Json 字符串（对应 Java `toUserDataString()`：
/// `agentid`/`userid`/`type` + `handle`）。
fn to_user_data_string(w: &WxCpAgentWorkBench) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("agentid".to_string(), serde_json::json!(w.agent_id));
    obj.insert(
        "userid".to_string(),
        serde_json::Value::from(w.user_id.as_str()),
    );
    obj.insert(
        "type".to_string(),
        serde_json::Value::from(w.r#type.as_str()),
    );
    handle(w, &mut obj, false);
    serde_json::Value::Object(obj).to_string()
}

/// 生成批量用户数据 Json 字符串（对应 Java `toBatchUserDataString()`：
/// `agentid`/`userid_list` + `handleBatch`）。
fn to_batch_user_data_string(w: &WxCpAgentWorkBench) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("agentid".to_string(), serde_json::json!(w.agent_id));
    obj.insert(
        "userid_list".to_string(),
        serde_json::Value::Array(
            w.userid_list
                .iter()
                .map(|v| serde_json::Value::from(v.as_str()))
                .collect(),
        ),
    );
    handle(w, &mut obj, true);
    serde_json::Value::Object(obj).to_string()
}

/// 处理不同类型的工作台数据（对应 Java `handle`/`handleBatch`；
/// 返回内容子对象；`batch=true` 时包进 `data` 子对象并带 `type`，
/// 否则以类型名为键挂到顶层）。
fn handle(
    w: &WxCpAgentWorkBench,
    obj: &mut serde_json::Map<String, serde_json::Value>,
    batch: bool,
) {
    let content: serde_json::Map<String, serde_json::Value> = match w.r#type.as_str() {
        "keydata" => {
            let items: Vec<serde_json::Value> = w
                .key_data_list
                .iter()
                .map(|k| {
                    serde_json::json!({
                        "key": k.key,
                        "data": k.data,
                        "jump_url": k.jump_url,
                        "pagepath": k.page_path,
                    })
                })
                .collect();
            let mut items_obj = serde_json::Map::new();
            items_obj.insert("items".to_string(), serde_json::Value::Array(items));
            items_obj
        }
        "image" => {
            let mut image = serde_json::Map::new();
            image.insert("url".to_string(), serde_json::Value::from(w.url.as_str()));
            image.insert(
                "jump_url".to_string(),
                serde_json::Value::from(w.jump_url.as_str()),
            );
            image.insert(
                "pagepath".to_string(),
                serde_json::Value::from(w.page_path.as_str()),
            );
            image
        }
        "list" => {
            let items: Vec<serde_json::Value> = w
                .lists
                .iter()
                .map(|l| {
                    serde_json::json!({
                        "title": l.title,
                        "jump_url": l.jump_url,
                        "pagepath": l.page_path,
                    })
                })
                .collect();
            let mut items_obj = serde_json::Map::new();
            items_obj.insert("items".to_string(), serde_json::Value::Array(items));
            items_obj
        }
        "webview" => {
            let mut webview = serde_json::Map::new();
            webview.insert("url".to_string(), serde_json::Value::from(w.url.as_str()));
            webview.insert(
                "jump_url".to_string(),
                serde_json::Value::from(w.jump_url.as_str()),
            );
            webview.insert(
                "pagepath".to_string(),
                serde_json::Value::from(w.page_path.as_str()),
            );
            if w.enable_webview_click {
                webview.insert(
                    "enable_webview_click".to_string(),
                    serde_json::Value::from(w.enable_webview_click),
                );
            }
            webview.insert(
                "height".to_string(),
                serde_json::Value::from(w.height.as_str()),
            );
            if w.hide_title {
                webview.insert(
                    "hide_title".to_string(),
                    serde_json::Value::from(w.hide_title),
                );
            }
            webview
        }
        _ => serde_json::Map::new(),
    };
    if batch {
        // handleBatch：内容包进 `data` 并带 `type`
        let mut data = serde_json::Map::new();
        data.insert(
            "type".to_string(),
            serde_json::Value::from(w.r#type.as_str()),
        );
        for (k, v) in content {
            data.insert(k, v);
        }
        obj.insert("data".to_string(), serde_json::Value::Object(data));
    } else {
        // handle：以类型名为键挂到顶层（Java `templateObject.add(type, items)`）
        obj.insert(w.r#type.clone(), serde_json::Value::Object(content));
    }
}
