//! 微信小商城 商品服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMinishopGoodsServiceImpl`
//! （`extends WxMaServiceImpl implements WxOpenMinishopGoodsService`）。
//!
//! Java 实现为上游桩代码：`getMinishopGoodsCat`/`addMinishopGoodsSPU`
//! 仅 `post(url, dto.toJsonObject().toString())` + `log.info` 后恒
//! `return null`。Rust 严格镜像：
//! - 请求体按 Java `toJsonObject()` 线格式手工拼装（注意 `AddMinishopGoodsSPU
//!   .toJsonObject()` 为上游 bug：`gson.toJson(...)` 内嵌为转义 JSON 字符串、
//!   `expressInfo` 为 camelCase 键，逐字镜像，见 [`minishop_goods_json`]）；
//! - 执行请求后返回 `Ok(None)` 镜像 `return null`（NOT_MIRRORED：
//!   Java 上游未实现响应解析，Rust 无法镜像语义，如实标注）。
//!
//! 依赖表达同其他 Ma 子域服务（`Weak<dyn WxOpenService>` + 按 appid 取回
//! 代 ma 桥接服务；Java 自身 extends WxMaServiceImpl，语义一致，ADAPTED）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMinishopGoodsService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::bean::AddMinishopGoodsSPU;
use crate::bean::GoodsCatList;
use crate::bean::ParentCatId;
use crate::bean::WxOpenResult;
use crate::enums::url_ma_domain::{minishop_goods_add_spu_url, minishop_goods_cat_url};

/// 微信小商城 商品服务实现（对应 Java `WxOpenMinishopGoodsServiceImpl`）。
pub struct WxOpenMinishopGoodsServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMinishopGoodsServiceImpl {
    /// 构建服务（对应 Java `extends WxMaServiceImpl` 的自持服务语义；
    /// Java 无显式构造，Rust 统一以门面弱引用 + appid 表达，ADAPTED）。
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 取代 ma 桥接服务（同
    /// [`WxOpenMaAuthServiceImpl::ma_service`]）。
    fn ma_service(&self) -> Result<Arc<dyn WxMaService>, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let any = component
            .get_wx_ma_service_by_appid(&self.app_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "getWxMaServiceByAppid 返回 None"))?;
        let ma = any.downcast::<WxOpenMaService>().map_err(|_| {
            WxErrorException::from_code(-99, "代 ma 服务 downcast 失败（缓存类型不匹配）")
        })?;
        Ok(ma as Arc<dyn WxMaService>)
    }
}

#[async_trait]
impl WxOpenMinishopGoodsService for WxOpenMinishopGoodsServiceImpl {
    /// 获取商品类目（对应 Java `getMinishopGoodsCat(ParentCatId dto)`：
    /// POST `dto.toJsonObject().toString()`，恒 `return null`）。
    async fn get_minishop_goods_cat(
        &self,
        f_cat_id: &ParentCatId,
    ) -> Result<Option<GoodsCatList>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "f_cat_id": f_cat_id.f_cat_id });
        let response = ma
            .post(&minishop_goods_cat_url(config.as_ref()), &body.to_string())
            .await?;
        // Java 仅 log.info(response) 后 return null
        let _ = response;
        Ok(None)
    }

    /// 新增商品 SPU（对应 Java `addMinishopGoodsSPU(AddMinishopGoodsSPU
    /// dto)`：POST `dto.toJsonObject().toString()`，恒 `return null`）。
    async fn add_minishop_goods_spu(
        &self,
        dto: &AddMinishopGoodsSPU,
    ) -> Result<Option<WxOpenResult>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = minishop_goods_json::add_spu(dto);
        let response = ma
            .post(&minishop_goods_add_spu_url(config.as_ref()), &body)
            .await?;
        // Java 仅 log.info(response) 后 return null
        let _ = response;
        Ok(None)
    }
}

/// 请求体拼装（逐字镜像 Java `toJsonObject()` 线格式）。
///
/// 注意：Java `addProperty(String, String)` 会把 `gson.toJson(...)` 的
/// JSON 再转义为 JSON 字符串内嵌（上游 bug），Rust 以
/// `serde_json::Value::String(serde_json::to_string(inner))` 复刻同一
/// 线格式；`brand_id` 为字符串数字（`brandId.toString()`）；
/// `expressInfo` 为 camelCase 键（Java 拼写，非 `express_info`）。
mod minishop_goods_json {
    use serde_json::Value;

    use crate::bean::AddMinishopGoodsSPU;

    pub(super) fn add_spu(dto: &AddMinishopGoodsSPU) -> String {
        // `gson.toJson(x)` 后作为字符串内嵌（Java addProperty(String, String)）
        let inner = |v: &Value| serde_json::to_string(v).unwrap_or_default();
        let body = serde_json::json!({
            "out_product_id": dto.out_product_id,
            "title": dto.title,
            "sub_title": dto.sub_title,
            "head_img": inner(&serde_json::to_value(&dto.head_img).unwrap_or_default()),
            "desc_info": inner(&serde_json::to_value(&dto.desc_info).unwrap_or_default()),
            "brand_id": dto.brand_id.to_string(),
            "cats": inner(&serde_json::to_value(&dto.cats).unwrap_or_default()),
            "attrs": inner(&serde_json::to_value(&dto.attrs).unwrap_or_default()),
            "model": dto.model,
            "expressInfo": inner(&serde_json::to_value(&dto.express_info).unwrap_or_default()),
            "skus": inner(&serde_json::to_value(&dto.skus).unwrap_or_default()),
        });
        body.to_string()
    }
}
