//! 人事助手服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpHrServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `cpService`），全部方法经门面 `post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - `userid` 空校验与 `fieldList` 空校验对应 Java
//!   `IllegalArgumentException`，以 `WxErrorException::from_code(-99, ...)`
//!   表达（ADAPTED）；
//! - `fields` 仅非空时写入（对应 Java `if (fields != null &&
//!   !fields.isEmpty())`）；`get_all` 恒写入（布尔值）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpHrService, WxCpService};
use crate::bean::{FieldItem, WxCpHrEmployeeFieldDataResp, WxCpHrEmployeeFieldInfoResp};
use crate::enums::url_hr;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 人事助手服务实现。
pub struct WxCpHrServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpHrServiceImpl {
    /// 构建人事助手服务（对应 Java 构造器注入 `WxCpService`）。
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

    /// 校验 userid（对应 Java `userid == null || userid.trim().isEmpty()`
    /// 抛 `IllegalArgumentException`）。
    fn validate_userid(userid: &str) -> Result<(), WxErrorException> {
        if userid.trim().is_empty() {
            return Err(WxErrorException::from_code(-99, "userid 不能为空"));
        }
        Ok(())
    }

    /// 组装 fields 数组（仅非空时写入，对应 Java `toJsonTree(fields)`）。
    fn put_fields(obj: &mut serde_json::Map<String, serde_json::Value>, fields: Option<&[&str]>) {
        if let Some(fields) = fields {
            if !fields.is_empty() {
                let arr: Vec<serde_json::Value> = fields
                    .iter()
                    .map(|v| serde_json::Value::String((*v).to_string()))
                    .collect();
                obj.insert("fields".to_string(), serde_json::Value::Array(arr));
            }
        }
    }
}

#[async_trait]
impl WxCpHrService for WxCpHrServiceImpl {
    /// 获取员工档案字段信息（对应 Java `getFieldInfo`）。
    async fn get_field_info(
        &self,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldInfoResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_fields(&mut obj, fields);
        let url = svc.wx_cp_config_storage().api_url(url_hr::GET_FIELD_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpHrEmployeeFieldInfoResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取员工档案数据（对应 Java
    /// `getEmployeeFieldInfo(String, List<String>)`）。
    async fn get_employee_field_info(
        &self,
        userid: &str,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldDataResp, WxErrorException> {
        self.get_employee_field_info_with_get_all(userid, false, fields)
            .await
    }

    /// 获取员工档案数据（对应 Java
    /// `getEmployeeFieldInfo(String, boolean, List<String>)`）。
    async fn get_employee_field_info_with_get_all(
        &self,
        userid: &str,
        get_all: bool,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldDataResp, WxErrorException> {
        Self::validate_userid(userid)?;
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        obj.insert("get_all".to_string(), serde_json::Value::Bool(get_all));
        Self::put_fields(&mut obj, fields);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_hr::GET_EMPLOYEE_FIELD_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpHrEmployeeFieldDataResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 更新员工档案数据（对应 Java `updateEmployeeFieldInfo`）。
    async fn update_employee_field_info(
        &self,
        userid: &str,
        field_list: &[FieldItem],
    ) -> Result<(), WxErrorException> {
        Self::validate_userid(userid)?;
        if field_list.is_empty() {
            return Err(WxErrorException::from_code(-99, "fieldList 不能为空"));
        }
        let svc = self.service()?;
        let field_list_json = serde_json::to_string(field_list)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        // Java：toJsonTree(fieldList) 直接作为 field_list 值
        let field_list_value: serde_json::Value = serde_json::from_str(&field_list_json)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        obj.insert("field_list".to_string(), field_list_value);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_hr::UPDATE_EMPLOYEE_FIELD_INFO);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：员工档案字段/数据查询请求体与响应解析、更新校验。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testGetFieldInfo`/`testGetEmployeeFieldInfo`：
    /// fields 仅非空时写入、get_all 恒写入、响应解析。
    #[tokio::test]
    async fn test_hr_get_field_info_and_employee() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/hr/get_fields") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","group_list":[{"group_id":1,"group_name":"基础信息","field_list":[{"fieldid":1,"field_name":"姓名","field_type":1}]}]}"#,
                )
            } else if path.contains("/cgi-bin/hr/get_staff_info") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","field_info":[{"fieldid":1,"field_name":"姓名","field_value":{"text":{"value":"张三"}}}]}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpHrServiceImpl::new(weak_service(&service));

        let field_info = svc_impl
            .get_field_info(Some(&["1", "2"]))
            .await
            .expect("获取字段信息成功");
        assert_eq!(field_info.group_list.len(), 1);
        let body = server.last_body();
        assert!(body.contains(r#""fields":["1","2"]"#), "body: {body}");
        assert!(server.last_path().contains("/cgi-bin/hr/get_fields"));

        // fields 为空时不写入（对应 Java `if (fields != null && !fields.isEmpty())`）
        let _ = svc_impl
            .get_field_info(None)
            .await
            .expect("获取字段信息成功");
        assert!(
            !server.last_body().contains("fields"),
            "body: {}",
            server.last_body()
        );

        // 员工档案数据：userid/get_all 恒写入
        let data = svc_impl
            .get_employee_field_info_with_get_all("zhangsan", true, None)
            .await
            .expect("获取员工档案成功");
        assert_eq!(data.field_info_list.len(), 1);
        let body = server.last_body();
        assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
        assert!(body.contains(r#""get_all":true"#), "body: {body}");
        assert!(server.last_path().contains("/cgi-bin/hr/get_staff_info"));

        // userid 为空 → 报错（Java IllegalArgumentException）
        assert!(svc_impl.get_employee_field_info("", None).await.is_err());
    }
}
