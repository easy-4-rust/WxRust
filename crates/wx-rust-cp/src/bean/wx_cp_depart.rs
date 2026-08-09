//! 企业微信部门。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpDepart`，线格式以
//! `util/json/WxCpDepartGsonAdapter` 为准：`id`/`name`/`name_en`/
//! `department_leader`/`parentid`/`order`；null 省略，
//! `department_leader` 非空数组才输出。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDepart {
    /// 部门 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i64>,
    /// 部门名称。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// 英文名称（wire `name_en`）。
    #[serde(rename = "name_en", skip_serializing_if = "Option::is_none", default)]
    pub en_name: Option<String>,
    /// 部门负责人的 UserID 列表（wire `department_leader`）。
    #[serde(
        rename = "department_leader",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub department_leader: Option<Vec<String>>,
    /// 父部门 id（wire `parentid`）。
    #[serde(rename = "parentid", skip_serializing_if = "Option::is_none", default)]
    pub parent_id: Option<i64>,
    /// 在父部门中的次序值（wire `order`）。
    #[serde(rename = "order", skip_serializing_if = "Option::is_none", default)]
    pub order: Option<i64>,
}

impl WxCpDepart {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDepart 解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDepart 序列化失败: {e}"))
    }
}
