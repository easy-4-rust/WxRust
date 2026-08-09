//! 授权方信息（授权方详情）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenAuthorizerInfoResult`。
//! 由 `WxOpenAuthorizerInfoResultGsonAdapter` 驱动解析
//! （`authorization_info`/`authorizer_info` 键），与字段名直映不同，
//! 故人工迁移。

use crate::bean::auth::{WxOpenAuthorizationInfo, WxOpenAuthorizerInfo};

/// 授权方信息（授权方详情）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenAuthorizerInfoResult {
    /// 授权信息。
    #[serde(rename = "authorization_info", default)]
    pub authorization_info: Option<WxOpenAuthorizationInfo>,
    /// 授权方基本信息。
    #[serde(rename = "authorizer_info", default)]
    pub authorizer_info: Option<WxOpenAuthorizerInfo>,
}

impl WxOpenAuthorizerInfoResult {
    /// 是否为小程序类型授权（对应 Java `isMiniProgram()`）。
    pub fn is_mini_program(&self) -> bool {
        self.authorizer_info
            .as_ref()
            .and_then(|i| i.mini_program_info.as_ref())
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java 测试 golden 线格式（WxOpenAuthorizerInfoResultTest）：
    /// snake_case 键、service/verify_type_info `{"id":N}` 扁平、
    /// func_info 数组扁平、MiniProgramInfo 大驼峰键。
    #[test]
    fn parse_java_golden() {
        let json = r#"{
  "authorizer_info": {
    "nick_name": "美妆饰品",
    "service_type_info": {"id": 0},
    "verify_type_info": {"id": -1},
    "user_name": "gh_c43395cb652e",
    "principal_name": "个人",
    "business_info": {"open_pay": 0, "open_shake": 0},
    "MiniProgramInfo": {
      "network": {"RequestDomain": ["https://weixin.qq.com"], "BizDomain": []},
      "categories": [{"first": "生活服务", "second": "丽人服务"}],
      "visit_status": 0
    },
    "register_type": 0,
    "account_status": 1,
    "basic_config": {"is_phone_configured": true, "is_email_configured": true}
  },
  "authorization_info": {
    "authorizer_appid": "wx326eecacf7370d4e",
    "authorizer_refresh_token": "refreshtoken@@@RU0Sgi7bD6apS7frS9gj8Sbws7OoDejK9Z-cm0EnCzg",
    "func_info": [
      {"funcscope_category": {"id": 3}},
      {"funcscope_category": {"id": 7}, "confirm_info": {"need_confirm": 0}}
    ]
  }
}"#;
        let res: WxOpenAuthorizerInfoResult = serde_json::from_str(json).unwrap();
        let info = res.authorizer_info.as_ref().unwrap();
        assert_eq!(info.nick_name.as_deref(), Some("美妆饰品"));
        assert_eq!(info.service_type_info, Some(0));
        assert_eq!(info.verify_type_info, Some(-1));
        assert_eq!(info.account_status, Some(1));
        assert!(info.mini_program_info.is_some());
        assert_eq!(
            info.mini_program_info
                .as_ref()
                .and_then(|m| m.network.as_ref())
                .and_then(|n| n.request_domain.as_ref())
                .map(|v| v.len()),
            Some(1)
        );
        assert!(res.is_mini_program());
        let auth = res.authorization_info.as_ref().unwrap();
        assert_eq!(auth.authorizer_appid.as_deref(), Some("wx326eecacf7370d4e"));
        assert_eq!(auth.func_info, vec![3, 7]);
    }
}
