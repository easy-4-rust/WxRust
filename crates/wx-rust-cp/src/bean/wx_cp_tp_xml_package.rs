//! 服务商推送 XML 包裹。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpXmlPackage`：服务商回调的
//! 外层 XML（`ToUserName`/`AgentID`/`Encrypt`），Java 以 XStream 解析
//! （golden：`WxCpTpXmlPackageTest`）。Rust 复用 `bean::message` 的
//! quick-xml 树解析。

use std::collections::HashMap;

use crate::bean::message::XmlValue;
use crate::bean::message::wx_cp_xml_message::{parse_tree, str_field};

/// 服务商推送 XML 包裹。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpTpXmlPackage {
    /// 存放所有 xml 属性和值的 map（对应 Java `allFieldsMap`）。
    pub all_fields_map: Option<HashMap<String, XmlValue>>,
    /// 企业微信 corpid（对应 Java `ToUserName`）。
    pub to_user_name: Option<String>,
    /// 应用 agentid（对应 Java `AgentID`）。
    pub agent_id: Option<String>,
    /// 加密的消息体（对应 Java `Encrypt`）。
    pub msg_encrypt: Option<String>,
}

impl WxCpTpXmlPackage {
    /// 从 xml 字符串解析（对应 Java `fromXml(String)`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let tree = parse_tree(xml)?;
        let root = match tree {
            XmlValue::Node(m) => m,
            other => {
                return Err(format!(
                    "XML 根元素应为节点，实际为: {}",
                    match other {
                        XmlValue::Scalar(s) => format!("标量 {s}"),
                        _ => "数组".to_string(),
                    }
                ));
            }
        };
        Ok(Self {
            all_fields_map: Some(root.clone()),
            to_user_name: str_field(&root, "ToUserName"),
            agent_id: str_field(&root, "AgentID"),
            msg_encrypt: str_field(&root, "Encrypt"),
        })
    }
}
