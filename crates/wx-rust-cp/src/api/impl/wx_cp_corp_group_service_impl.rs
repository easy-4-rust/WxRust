//! 企业互联服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpCorpGroupServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `cpService`），经门面 `post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - 请求体五个字段（`agentid`/`corpid`/`business_type`/`limit`/`cursor`）
//!   无条件写入（对应 Java `JsonObject.addProperty`，null 写 null）；
//! - 响应解析 `corp_list` 数组（对应 Java
//!   `TypeToken<List<WxCpCorpGroupCorp>>`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpCorpGroupService, WxCpService};
use crate::bean::WxCpCorpGroupCorp;
use crate::enums::url_corp_group;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 企业互联服务实现。
pub struct WxCpCorpGroupServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpCorpGroupServiceImpl {
    /// 构建企业互联服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }
}

#[async_trait]
impl WxCpCorpGroupService for WxCpCorpGroupServiceImpl {
    /// 获取应用共享信息列表（对应 Java `listAppShareInfo`）。
    async fn list_app_share_info(
        &self,
        agent_id: Option<i32>,
        business_type: Option<i32>,
        corp_id: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<Vec<WxCpCorpGroupCorp>, WxErrorException> {
        let svc = self.service()?;
        // Java：五个字段无条件 addProperty（null 写 null）
        let mut obj = serde_json::Map::new();
        obj.insert(
            "agentid".to_string(),
            agent_id
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "corpid".to_string(),
            corp_id
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "business_type".to_string(),
            business_type
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "limit".to_string(),
            limit
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "cursor".to_string(),
            cursor
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_corp_group::LIST_SHARE_APP_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let arr = json
            .get("corp_list")
            .and_then(|v| v.as_array())
            .ok_or_else(|| WxErrorException::from_code(-99, "corp_list 字段缺失"))?;
        serde_json::from_value(serde_json::Value::Array(arr.clone()))
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：企业互联应用共享信息列表请求体（无条件写 null 字段）与
    //! corp_list 解析。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testListAppShareInfo`：请求体含 agentid/corpid/
    /// business_type/limit/cursor（null 写 null），解析 corp_list。
    #[tokio::test]
    async fn test_corp_group_list_app_share_info() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/corpgroup/corp/list_app_share_info") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","corp_list":[{"corpid":"ww123","corp_name":"下游企业","agentid":1000002}]}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpCorpGroupServiceImpl::new(weak_service(&service));

        let list = svc_impl
            .list_app_share_info(
                Some(1000002),
                Some(1),
                Some("ww123"),
                Some(100),
                Some("CURSOR"),
            )
            .await
            .expect("获取应用共享信息成功");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].corpid, "ww123");
        assert_eq!(list[0].corp_name, "下游企业");
        assert_eq!(list[0].agentid, 1000002);
        let body = server.last_body();
        assert!(body.contains(r#""agentid":1000002"#), "body: {body}");
        assert!(body.contains(r#""corpid":"ww123""#), "body: {body}");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/corpgroup/corp/list_app_share_info")
        );

        // None 参数写 null（对应 Java 无条件 addProperty）
        let _ = svc_impl
            .list_app_share_info(None, None, None, None, None)
            .await
            .expect("获取成功");
        let body = server.last_body();
        assert!(body.contains(r#""agentid":null"#), "body: {body}");
        assert!(body.contains(r#""corpid":null"#), "body: {body}");
        assert!(body.contains(r#""limit":null"#), "body: {body}");
    }
}
