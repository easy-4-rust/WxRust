//! 企业微信第三方应用 ID 转换服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpIdConvertService`：
//! 企业微信三方应用 ID 转换接口（unionid↔external_userid、客户标签 ID、
//! 微信客服 ID、临时外部用户 ID）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpTpConvertTmpExternalUserIdResult, WxCpTpOpenKfIdConvertResult,
    WxCpTpTagIdListConvertResult, WxCpTpUnionidToExternalUseridResult,
};

/// 企业微信第三方应用 ID 转换服务。
#[async_trait]
pub trait WxCpTpIdConvertService: Send + Sync {
    /// unionid 与 external_userid 的关联（对应 Java
    /// `unionidToExternalUserid(String, String, String, Integer)`；
    /// subjectType：0 主体名称是企业的，1 主体名称是服务商的）。
    async fn unionid_to_external_userid(
        &self,
        corp_id: &str,
        unionid: &str,
        openid: &str,
        subject_type: Option<i32>,
    ) -> Result<WxCpTpUnionidToExternalUseridResult, WxErrorException>;

    /// 将企业主体下的客户标签 ID 转换成服务商主体下的客户标签 ID（对应
    /// Java `externalTagId(String, String...)`，最多 1000 个）。
    async fn external_tag_id(
        &self,
        corp_id: &str,
        external_tag_id_list: &[String],
    ) -> Result<WxCpTpTagIdListConvertResult, WxErrorException>;

    /// 将企业主体下的微信客服 ID 转换成服务商主体下的微信客服 ID（对应
    /// Java `ConvertOpenKfId(String, String...)`，最多 1000 个）。
    async fn convert_open_kf_id(
        &self,
        corp_id: &str,
        open_kf_id_list: &[String],
    ) -> Result<WxCpTpOpenKfIdConvertResult, WxErrorException>;

    /// 将应用获取的外部用户临时 id（tmp_external_userid）转换为
    /// external_userid（对应 Java `convertTmpExternalUserId(String, int,
    /// int, String...)`；businessType：1-会议 2-收集表；userType：
    /// 1-客户 2-企业互联 3-上下游 4-互联企业；最多 100 个）。
    async fn convert_tmp_external_user_id(
        &self,
        corp_id: &str,
        business_type: i32,
        user_type: i32,
        tmp_external_user_id_list: &[String],
    ) -> Result<WxCpTpConvertTmpExternalUserIdResult, WxErrorException>;
}
