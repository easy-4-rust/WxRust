//! 企业微信 OA 服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `mainService`），全部方法经门面 `get`/`post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - 用户列表上限 100（`USER_IDS_LIMIT`）与一个月/30 天时间跨度校验
//!   （`MONTH_SECONDS = 31 * 24 * 60 * 60`）对应 Java `WxRuntimeException`，
//!   以 `WxErrorException::from_code(-99, ...)` 表达（ADAPTED）；
//! - 审批单号分页 `size` 校验（0 < size <= 100）对应 Java
//!   `IllegalArgumentException`；
//! - 打卡数据/日报/月报/排班与公费电话记录等响应中的嵌套数组
//!   （`checkindata`/`info`/`group`/`datas`/`schedule_list`/`record`）以
//!   `serde_json::from_value` 解析（Java `TypeToken<List<...>>`，ADAPTED）；
//! - Java `Date` 以 `chrono::DateTime<Utc>` 表达（trait 冻结签名），
//!   时间戳取 `timestamp()` 秒。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaService, WxCpService};
use crate::bean::{
    WxCpApprovalDetailResult, WxCpApprovalInfo, WxCpApprovalInfoQueryFilter, WxCpBaseResp,
    WxCpCheckinData, WxCpCheckinDayData, WxCpCheckinMonthData, WxCpCheckinOption,
    WxCpCheckinSchedule, WxCpCorpConfInfo, WxCpCropCheckinOption, WxCpDialRecord,
    WxCpGetApprovalData, WxCpOaApplyEventRequest, WxCpOaApprovalTemplate,
    WxCpOaApprovalTemplateResult, WxCpSetCheckinSchedule, WxCpUserVacationQuota,
};
use crate::enums::url_oa;

/// 时间跨度上限：31 天（秒），对应 Java `MONTH_SECONDS`。
const MONTH_SECONDS: i64 = 31 * 24 * 60 * 60;
/// 用户列表数量上限，对应 Java `USER_IDS_LIMIT`。
const USER_IDS_LIMIT: usize = 100;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 企业微信 OA 服务实现。
pub struct WxCpOaServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaServiceImpl {
    /// 构建 OA 服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `mainService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 序列化请求对象（对应 Java `WxCpGsonBuilder.toJson`）。
    fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 校验用户列表（对应 Java `getCheckinXxx` 系列的用户数校验）。
    fn validate_user_ids(user_id_list: &[&str]) -> Result<(), WxErrorException> {
        if user_id_list.is_empty() || user_id_list.len() > USER_IDS_LIMIT {
            return Err(WxErrorException::from_code(
                -99,
                "用户列表不能为空，不超过 100 个，若用户超过 100 个，请分批获取",
            ));
        }
        Ok(())
    }

    /// 解析响应中的嵌套数组字段（对应 Java
    /// `GsonParser.parse(responseContent).get(key)` +
    /// `TypeToken<List<...>>`）。
    fn parse_array<T>(response: &str, key: &str) -> Result<Vec<T>, WxErrorException>
    where
        T: serde::de::DeserializeOwned,
    {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let arr = json
            .get(key)
            .and_then(|v| v.as_array())
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{key} 字段缺失")))?;
        serde_json::from_value(serde_json::Value::Array(arr.clone()))
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 从响应提取字符串字段（对应 Java `GsonParser.parse(...).get(...)
    /// .getAsString()`）。
    fn extract_string(response: &str, field: &str) -> Result<String, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get(field)
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{field} 字段缺失")))
    }

    /// 组装 useridlist 数组（对应 Java `JsonArray.add` 循环）。
    fn user_id_array(user_id_list: &[&str]) -> Vec<serde_json::Value> {
        user_id_list
            .iter()
            .map(|v| serde_json::Value::String((*v).to_string()))
            .collect()
    }
}

#[async_trait]
impl WxCpOaService for WxCpOaServiceImpl {
    /// 提交审批申请（对应 Java `apply`）。
    async fn apply(&self, request: &WxCpOaApplyEventRequest) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::APPLY_EVENT);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        Self::extract_string(&response, "sp_no")
    }

    /// 获取打卡数据（对应 Java `getCheckinData`）。
    async fn get_checkin_data(
        &self,
        open_checkin_data_type: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinData>, WxErrorException> {
        Self::validate_user_ids(user_id_list)?;
        let end_timestamp = end_time.timestamp();
        let start_timestamp = start_time.timestamp();
        if end_timestamp - start_timestamp < 0 || end_timestamp - start_timestamp > MONTH_SECONDS {
            return Err(WxErrorException::from_code(
                -99,
                "获取记录时间跨度不超过一个月",
            ));
        }
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "opencheckindatatype".to_string(),
            serde_json::Value::from(open_checkin_data_type),
        );
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_timestamp),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_timestamp),
        );
        obj.insert(
            "useridlist".to_string(),
            serde_json::Value::Array(Self::user_id_array(user_id_list)),
        );
        let url = svc.wx_cp_config_storage().api_url(url_oa::GET_CHECKIN_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpCheckinData>(&response, "checkindata")
    }

    /// 获取打卡规则（对应 Java `getCheckinOption`）。
    async fn get_checkin_option(
        &self,
        datetime: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinOption>, WxErrorException> {
        Self::validate_user_ids(user_id_list)?;
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "datetime".to_string(),
            serde_json::Value::from(datetime.timestamp()),
        );
        obj.insert(
            "useridlist".to_string(),
            serde_json::Value::Array(Self::user_id_array(user_id_list)),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_CHECKIN_OPTION);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpCheckinOption>(&response, "info")
    }

    /// 获取企业所有打卡规则（对应 Java `getCropCheckinOption`）。
    async fn get_crop_checkin_option(
        &self,
    ) -> Result<Vec<WxCpCropCheckinOption>, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_CORP_CHECKIN_OPTION);
        let response = svc.post(&url, "{}").await?;
        Self::parse_array::<WxCpCropCheckinOption>(&response, "group")
    }

    /// 批量获取审批单号（旧分页游标版，对应 Java
    /// `getApprovalInfo(Date, Date, Integer, Integer, List)`，`@Deprecated`）。
    async fn get_approval_info_with_cursor(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        cursor: Option<i32>,
        size: Option<i32>,
        filters: Option<&[WxCpApprovalInfoQueryFilter]>,
    ) -> Result<WxCpApprovalInfo, WxErrorException> {
        let cursor = cursor.unwrap_or(0);
        let size = size.unwrap_or(100);
        if !(0..=100).contains(&size) {
            return Err(WxErrorException::from_code(
                -99,
                "size参数错误,请使用[1-100]填充，默认100",
            ));
        }
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        obj.insert("size".to_string(), serde_json::Value::from(size));
        obj.insert("cursor".to_string(), serde_json::Value::from(cursor));
        if let Some(filters) = filters {
            if !filters.is_empty() {
                let mut filter_json_array = Vec::new();
                for filter in filters {
                    let filter_json = Self::to_json(filter)?;
                    let value: serde_json::Value = serde_json::from_str(&filter_json)
                        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                    filter_json_array.push(value);
                }
                obj.insert(
                    "filters".to_string(),
                    serde_json::Value::Array(filter_json_array),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_APPROVAL_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 批量获取审批单号（简版，对应 Java
    /// `getApprovalInfo(Date, Date)`，`@Deprecated`）。
    async fn get_approval_info(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<WxCpApprovalInfo, WxErrorException> {
        self.get_approval_info_with_cursor(start_time, end_time, Some(0), None, None)
            .await
    }

    /// 批量获取审批单号（新分页游标版，对应 Java
    /// `getApprovalInfo(Date, Date, String, Integer, List)`）。
    async fn get_approval_info_with_new_cursor(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        new_cursor: Option<&str>,
        size: Option<i32>,
        filters: Option<&[WxCpApprovalInfoQueryFilter]>,
    ) -> Result<WxCpApprovalInfo, WxErrorException> {
        // Java：newCursor 默认空串
        let new_cursor = new_cursor.unwrap_or("");
        let size = size.unwrap_or(100);
        if !(0..=100).contains(&size) {
            return Err(WxErrorException::from_code(
                -99,
                "size参数错误,请使用[1-100]填充，默认100",
            ));
        }
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        obj.insert("size".to_string(), serde_json::Value::from(size));
        obj.insert(
            "new_cursor".to_string(),
            serde_json::Value::String(new_cursor.to_string()),
        );
        if let Some(filters) = filters {
            if !filters.is_empty() {
                let mut filter_json_array = Vec::new();
                for filter in filters {
                    let filter_json = Self::to_json(filter)?;
                    let value: serde_json::Value = serde_json::from_str(&filter_json)
                        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                    filter_json_array.push(value);
                }
                obj.insert(
                    "filters".to_string(),
                    serde_json::Value::Array(filter_json_array),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_APPROVAL_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取审批申请详情（对应 Java `getApprovalDetail`）。
    async fn get_approval_detail(
        &self,
        sp_no: &str,
    ) -> Result<WxCpApprovalDetailResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "sp_no".to_string(),
            serde_json::Value::String(sp_no.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_APPROVAL_DETAIL);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取企业假期管理配置（对应 Java `getCorpConf`）。
    async fn get_corp_conf(&self) -> Result<WxCpCorpConfInfo, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::GET_CORP_CONF);
        let response = svc.get(&url, "").await?;
        WxCpCorpConfInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取成员假期余额（对应 Java `getUserVacationQuota`）。
    async fn get_user_vacation_quota(
        &self,
        user_id: &str,
    ) -> Result<WxCpUserVacationQuota, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_USER_VACATION_QUOTA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserVacationQuota::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取审批数据（旧，对应 Java `getApprovalData`）。
    async fn get_approval_data(
        &self,
        start_time: i64,
        end_time: i64,
        next_sp_num: Option<i64>,
    ) -> Result<WxCpGetApprovalData, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert("starttime".to_string(), serde_json::Value::from(start_time));
        obj.insert("endtime".to_string(), serde_json::Value::from(end_time));
        if let Some(next_sp_num) = next_sp_num {
            obj.insert(
                "next_spnum".to_string(),
                serde_json::Value::from(next_sp_num),
            );
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_APPROVAL_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetApprovalData::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 修改成员假期余额（对应 Java `setOneUserQuota`）。
    async fn set_one_user_quota(
        &self,
        user_id: &str,
        vacation_id: i32,
        left_duration: i32,
        time_attr: i32,
        remarks: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        obj.insert(
            "vacation_id".to_string(),
            serde_json::Value::from(vacation_id),
        );
        obj.insert(
            "leftduration".to_string(),
            serde_json::Value::from(left_duration),
        );
        obj.insert("time_attr".to_string(), serde_json::Value::from(time_attr));
        if let Some(remarks) = remarks {
            if !remarks.is_empty() {
                obj.insert(
                    "remarks".to_string(),
                    serde_json::Value::String(remarks.to_string()),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::SET_ONE_USER_QUOTA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取公费电话拨打记录（对应 Java `getDialRecord`）。
    async fn get_dial_record(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<WxCpDialRecord>, WxErrorException> {
        let offset = offset.unwrap_or(0);
        let limit = if let Some(limit) = limit {
            if limit <= 0 { 100 } else { limit }
        } else {
            100
        };
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert("offset".to_string(), serde_json::Value::from(offset));
        obj.insert("limit".to_string(), serde_json::Value::from(limit));
        // Java `if (startTime != null && endTime != null)`：trait 签名必传，
        // 恒进入时间校验分支
        let end_timestamp = end_time.timestamp();
        let start_timestamp = start_time.timestamp();
        if end_timestamp - start_timestamp < 0 || end_timestamp - start_timestamp >= MONTH_SECONDS {
            return Err(WxErrorException::from_code(
                -99,
                "受限于网络传输，起止时间的最大跨度为30天，如超过30天，则以结束时间为基准向前取30天进行查询",
            ));
        }
        obj.insert(
            "start_time".to_string(),
            serde_json::Value::from(start_timestamp),
        );
        obj.insert(
            "end_time".to_string(),
            serde_json::Value::from(end_timestamp),
        );
        let url = svc.wx_cp_config_storage().api_url(url_oa::GET_DIAL_RECORD);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpDialRecord>(&response, "record")
    }

    /// 获取审批模板详情（对应 Java `getTemplateDetail`）。
    async fn get_template_detail(
        &self,
        template_id: &str,
    ) -> Result<WxCpOaApprovalTemplateResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "template_id".to_string(),
            serde_json::Value::String(template_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_TEMPLATE_DETAIL);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpOaApprovalTemplateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 创建审批模板（对应 Java `createOaApprovalTemplate`）。
    async fn create_oa_approval_template(
        &self,
        cp_template: &WxCpOaApprovalTemplate,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::CREATE_TEMPLATE);
        let response = svc.post(&url, &Self::to_json(cp_template)?).await?;
        Self::extract_string(&response, "template_id")
    }

    /// 更新审批模板（对应 Java `updateOaApprovalTemplate`）。
    async fn update_oa_approval_template(
        &self,
        wx_cp_template: &WxCpOaApprovalTemplate,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::UPDATE_TEMPLATE);
        svc.post(&url, &Self::to_json(wx_cp_template)?).await?;
        Ok(())
    }

    /// 获取打卡日报数据（对应 Java `getCheckinDayData`）。
    async fn get_checkin_day_data(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinDayData>, WxErrorException> {
        Self::validate_user_ids(user_id_list)?;
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        obj.insert(
            "useridlist".to_string(),
            serde_json::Value::Array(Self::user_id_array(user_id_list)),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_CHECKIN_DAY_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpCheckinDayData>(&response, "datas")
    }

    /// 获取打卡月报数据（对应 Java `getCheckinMonthData`）。
    async fn get_checkin_month_data(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinMonthData>, WxErrorException> {
        Self::validate_user_ids(user_id_list)?;
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        obj.insert(
            "useridlist".to_string(),
            serde_json::Value::Array(Self::user_id_array(user_id_list)),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_CHECKIN_MONTH_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpCheckinMonthData>(&response, "datas")
    }

    /// 获取打卡人员排班信息（对应 Java `getCheckinScheduleList`）。
    async fn get_checkin_schedule_list(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinSchedule>, WxErrorException> {
        Self::validate_user_ids(user_id_list)?;
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "starttime".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "endtime".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        obj.insert(
            "useridlist".to_string(),
            serde_json::Value::Array(Self::user_id_array(user_id_list)),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_CHECKIN_SCHEDULE_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::parse_array::<WxCpCheckinSchedule>(&response, "schedule_list")
    }

    /// 为打卡人员排班（对应 Java `setCheckinScheduleList`）。
    async fn set_checkin_schedule_list(
        &self,
        wx_cp_set_checkin_schedule: &WxCpSetCheckinSchedule,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::SET_CHECKIN_SCHEDULE_DATA);
        svc.post(&url, &Self::to_json(wx_cp_set_checkin_schedule)?)
            .await?;
        Ok(())
    }

    /// 录入打卡人员人脸信息（对应 Java `addCheckInUserFace`）。
    async fn add_check_in_user_face(
        &self,
        user_id: &str,
        user_face: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        obj.insert(
            "userface".to_string(),
            serde_json::Value::String(user_face.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::ADD_CHECK_IN_USER_FACE);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：经 MockServer 验证 OA 接口请求路径/请求体/响应解析
    //! （镜像 Java `WxCpOaServiceImplTest` 的有效用例语义）。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    fn ts(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(secs, 0).expect("合法时间戳")
    }

    /// 镜像 Java `testApply`：提交审批申请，响应提取 sp_no。
    #[tokio::test]
    async fn test_oa_apply() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/oa/applyevent") {
                json(r#"{"errcode":0,"errmsg":"ok","sp_no":"201906010001"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpOaServiceImpl::new(weak_service(&service));

        let mut request = WxCpOaApplyEventRequest::default();
        request.creator_user_id = "zhangsan".to_string();
        request.template_id = "3Tkq4d2oX".to_string();
        let sp_no = svc_impl.apply(&request).await.expect("提交审批成功");
        assert_eq!(sp_no, "201906010001");
        let body = server.last_body();
        assert!(
            body.contains(r#""creator_userid":"zhangsan""#),
            "body: {body}"
        );
        assert!(
            body.contains(r#""template_id":"3Tkq4d2oX""#),
            "body: {body}"
        );
        assert!(server.last_path().contains("/cgi-bin/oa/applyevent"));
    }

    /// 镜像 Java `testGetCheckinData`：打卡数据请求体（opencheckindatatype/
    /// starttime/endtime/useridlist）与 checkindata 数组解析；时间跨度
    /// 超过一个月报错。
    #[tokio::test]
    async fn test_oa_get_checkin_data() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/checkin/getcheckindata") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","checkindata":[{"userid":"zhangsan","checkin_time":1600000000,"sch_checkin_time":1599990000}]}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpOaServiceImpl::new(weak_service(&service));

        let data = svc_impl
            .get_checkin_data(3, ts(1600000000), ts(1600003600), &["zhangsan"])
            .await
            .expect("获取打卡数据成功");
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].user_id, "zhangsan");
        assert_eq!(data[0].checkin_time, 1600000000);
        let body = server.last_body();
        assert!(body.contains(r#""opencheckindatatype":3"#), "body: {body}");
        assert!(body.contains(r#""starttime":1600000000"#), "body: {body}");
        assert!(
            body.contains(r#""useridlist":["zhangsan"]"#),
            "body: {body}"
        );
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/checkin/getcheckindata")
        );

        // 时间跨度超过一个月（31 天）→ 报错（Java WxRuntimeException）
        assert!(
            svc_impl
                .get_checkin_data(
                    3,
                    ts(1600000000),
                    ts(1600000000 + MONTH_SECONDS + 1),
                    &["zhangsan"]
                )
                .await
                .is_err()
        );
        // 用户列表为空 → 报错
        assert!(
            svc_impl
                .get_checkin_data(3, ts(1600000000), ts(1600003600), &[])
                .await
                .is_err()
        );
    }

    /// 镜像 Java `testGetApprovalInfo`（新分页游标版）：new_cursor/size/
    /// filters 请求体与 sp_no_list 解析。
    #[tokio::test]
    async fn test_oa_get_approval_info_new_cursor() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/oa/getapprovalinfo") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","sp_no_list":["201906010001","201906010002"],"new_next_cursor":"NEXT"}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpOaServiceImpl::new(weak_service(&service));

        let filters = [WxCpApprovalInfoQueryFilter {
            key: crate::bean::oa::wx_cp_approval_info_query_filter::Key::SpStatus,
            value: serde_json::json!([2, 3]),
        }];
        let info = svc_impl
            .get_approval_info_with_new_cursor(
                ts(1600000000),
                ts(1600003600),
                Some("CURSOR"),
                Some(10),
                Some(&filters),
            )
            .await
            .expect("获取审批单号成功");
        assert_eq!(
            info.sp_no_list,
            vec!["201906010001".to_string(), "201906010002".to_string()]
        );
        assert_eq!(info.new_next_cursor, "NEXT");
        let body = server.last_body();
        assert!(body.contains(r#""new_cursor":"CURSOR""#), "body: {body}");
        assert!(body.contains(r#""size":10"#), "body: {body}");
        assert!(body.contains(r#""key":"sp_status""#), "body: {body}");
        assert!(server.last_path().contains("/cgi-bin/oa/getapprovalinfo"));

        // size 越界 → 报错（Java IllegalArgumentException）
        assert!(
            svc_impl
                .get_approval_info_with_new_cursor(
                    ts(1600000000),
                    ts(1600003600),
                    None,
                    Some(101),
                    None
                )
                .await
                .is_err()
        );
    }
}
