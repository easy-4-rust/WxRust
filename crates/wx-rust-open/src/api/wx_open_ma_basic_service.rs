//! 小程序基础信息服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaBasicService`。
//! 微信第三方平台 小程序基础信息接口（小程序名称、头像、描述、类目等
//! 信息设置），文档：
//! <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/category/getallcategories.html>
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_*_url`，api_host
//! 前缀模式；`component_rebind_admin_url` 为 mp.weixin.qq.com 固定
//! 格式化串）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxFastMaAccountBasicInfoResult;
use crate::bean::WxFastMaBeenSetCategoryResult;
use crate::bean::WxFastMaCategory;
use crate::bean::WxFastMaCheckNickameResult;
use crate::bean::WxFastMaQueryNicknameStatusResult;
use crate::bean::WxFastMaSetNickameResult;
use crate::bean::WxOpenGetAllCategoriesByTypeResult;
use crate::bean::WxOpenMaCategoryNameListResult;
use crate::bean::WxOpenMaGetOrderPathResult;
use crate::bean::WxOpenResult;

/// 微信第三方平台 小程序基础信息服务（对应 Java `WxOpenMaBasicService`）。
#[async_trait]
pub trait WxOpenMaBasicService: Send + Sync {
    /// 获取小程序的信息（对应 Java `getAccountBasicInfo()`，GET 请求）。
    async fn get_account_basic_info(
        &self,
    ) -> Result<WxFastMaAccountBasicInfoResult, WxErrorException>;

    /// 小程序名称设置及改名（对应 Java `setNickname(String nickname,
    /// String idCard, String license, String namingOtherStuff1, String
    /// namingOtherStuff2)`）。
    ///
    /// 若接口未返回 audit_id，说明名称已直接设置成功，无需审核；若返回
    /// audit_id 则名称正在审核中。`id_card` 身份证照片临时素材 mediaid
    /// （个人号必填）；`license` 组织机构代码证或营业执照临时素材 mediaid
    /// （组织号必填）；`naming_other_stuff_1/2` 其他证明材料 mediaid。
    async fn set_nickname(
        &self,
        nickname: &str,
        id_card: &str,
        license: &str,
        naming_other_stuff1: &str,
        naming_other_stuff2: &str,
    ) -> Result<WxFastMaSetNickameResult, WxErrorException>;

    /// 小程序改名审核状态查询（对应 Java
    /// `querySetNicknameStatus(String auditId)`）。
    async fn query_set_nickname_status(
        &self,
        audit_id: &str,
    ) -> Result<WxFastMaQueryNicknameStatusResult, WxErrorException>;

    /// 微信认证名称检测（对应 Java
    /// `checkWxVerifyNickname(String nickname)`）。
    async fn check_wx_verify_nickname(
        &self,
        nickname: &str,
    ) -> Result<WxFastMaCheckNickameResult, WxErrorException>;

    /// 修改头像（对应 Java `modifyHeadImage(String headImgMediaId, float
    /// x1, float y1, float x2, float y2)`）。
    ///
    /// 图片格式只支持 BMP/JPEG/JPG/GIF/PNG，大小不超过 2M；实际头像
    /// 始终为正方形。`x1/y1` 裁剪框左上角坐标、`x2/y2` 右下角坐标
    /// （取值范围 [0, 1]）。
    async fn modify_head_image(
        &self,
        head_img_media_id: &str,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 修改功能介绍（对应 Java `modifySignature(String signature)`，
    /// 简介 4-120 字）。
    async fn modify_signature(&self, signature: &str) -> Result<WxOpenResult, WxErrorException>;

    /// 获取换绑管理员 URL（对应 Java
    /// `getComponentRebindAdminUrl(String redirectUri, String appId)`）。
    ///
    /// Java 以 `URLEncoder.encode(redirectUri, "UTF-8")`（表单编码：
    /// 空格 → `+`）编码后按 `URL_COMPONENT_REBIND_ADMIN` 格式化
    /// （`%s`：appid / component_appid / 编码后 redirect_uri），纯字符串
    /// 构建不抛错（Java `@SneakyThrows` 吞掉 UnsupportedEncodingException）。
    fn get_component_rebind_admin_url(&self, redirect_uri: &str, app_id: &str) -> String;

    /// 管理员换绑（对应 Java `componentRebindAdmin(String taskId)`；
    /// `task_id` 为公众平台最终点击提交回跳到第三方平台时携带的换绑
    /// 管理员任务序列号）。
    async fn component_rebind_admin(&self, task_id: &str)
    -> Result<WxOpenResult, WxErrorException>;

    /// 获取账号可以设置的所有类目（对应 Java `getAllCategories()`，
    /// 直接返回原始字符串）。
    ///
    /// 因为不同类目含有特定字段，目前没有完整的类目信息数据，为保证
    /// 兼容性，Java 放弃将 response 转换为实体。
    async fn get_all_categories(&self) -> Result<String, WxErrorException>;

    /// 获取不同类型主体可设置的类目（对应 Java
    /// `getAllCategoriesByType(String verifyType)`）。
    async fn get_all_categories_by_type(
        &self,
        verify_type: &str,
    ) -> Result<WxOpenGetAllCategoriesByTypeResult, WxErrorException>;

    /// 添加类目（对应 Java `addCategory(List<WxFastMaCategory>
    /// categoryList)`）。
    async fn add_category(
        &self,
        category_list: &[WxFastMaCategory],
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 删除类目（对应 Java `deleteCategory(int first, int second)`；
    /// `first` 一级类目 ID，`second` 二级类目 ID）。
    async fn delete_category(
        &self,
        first: i32,
        second: i32,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 获取账号已经设置的所有类目（对应 Java `getCategory()`，GET 请求）。
    async fn get_category(&self) -> Result<WxFastMaBeenSetCategoryResult, WxErrorException>;

    /// 修改类目（对应 Java `modifyCategory(WxFastMaCategory category)`）。
    async fn modify_category(
        &self,
        category: &WxFastMaCategory,
    ) -> Result<WxOpenResult, WxErrorException>;

    /// 获取类目名称信息（对应 Java `getAllCategoryName()`，GET 请求；
    /// 用于给用户展示选择）。
    async fn get_all_category_name(
        &self,
    ) -> Result<WxOpenMaCategoryNameListResult, WxErrorException>;

    /// 获取订单页 path 信息（对应 Java `getOrderPathInfo(int infoType)`；
    /// `info_type`：0 线上版，1 审核版）。
    async fn get_order_path_info(
        &self,
        info_type: i32,
    ) -> Result<WxOpenMaGetOrderPathResult, WxErrorException>;
}
