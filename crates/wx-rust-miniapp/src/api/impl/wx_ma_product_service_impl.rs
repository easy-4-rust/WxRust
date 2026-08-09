//! 小程序交易组件-标准版商品服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaProductServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaProductService;
use crate::bean::product::{
    WxMinishopGetBrandResponse, WxMinishopGetCategoryResponse, WxMinishopGetFrightTemplateResponse,
    WxMinishopResult, WxMinishopSku, WxMinishopSkuListResponse, WxMinishopSpu,
    WxMinishopSpuGetResponse, WxMinishopSpuListResponse,
};
use crate::bean::shop::request::WxMaShopSpuPageRequest;
use crate::bean::shop::response::WxMaShopBaseResponse;
use crate::config::DEFAULT_API_HOST_URL;
use crate::enums::g3_urls::url_g3_shop::product as product_url;

/// 构建 JSON 对象（跳过空值，对应 Java `GsonHelper.buildJsonObject`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 从响应 `data` 对象中构造 `WxMinishopResult`（对应 Java
/// `WxMinishopResult<...>` 手工组装逻辑：仅设置 errcode 与 data）。
fn build_result(data: serde_json::Value) -> WxMinishopResult {
    WxMinishopResult {
        errcode: 0,
        errmsg: String::new(),
        data,
    }
}

/// 小程序交易组件-标准版商品服务实现。
pub struct WxMaProductServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaProductServiceImpl {
    /// 构建标准版商品服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 注入 access_token 并做自定义域名替换（对应 Java `executeInternal` 的
    /// token 注入 + `getEffectiveApiHostUrl()` 替换语义）。
    async fn build_url(svc: &dyn WxMaService, url: &str) -> Result<String, WxErrorException> {
        let config = svc.wx_ma_config();
        let access_token = svc.get_access_token().await?;
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != DEFAULT_API_HOST_URL {
            url.replace(DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url.to_string()
        };
        // Java `execute`：uri 已有查询参数时以 `&` 追加 access_token
        let token_param = if url.contains('?') {
            format!("&access_token={access_token}")
        } else {
            format!("?access_token={access_token}")
        };
        Ok(format!("{url}{token_param}"))
    }
}

#[async_trait]
impl WxMaProductService for WxMaProductServiceImpl {
    /// 对应 Java `WxMaProductServiceImpl.uploadImg(File, Integer, Integer, Integer)`：
    /// URL 拼 `?upload_type=0&height=&width=&resp_type=`，multipart 上传 media 文件，
    /// 校验 errcode 后解析 `WxMinishopImageUploadResult`。
    async fn upload_img(
        &self,
        file_path: &str,
        resp_type: i32,
        width: i32,
        height: i32,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let base = product_url::other::img_upload_url(config.as_ref());
        let url = Self::build_url(
            svc.as_ref(),
            &format!("{base}?upload_type=0&height={height}&width={width}&resp_type={resp_type}"),
        )
        .await?;
        let file_bytes = std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".to_string());
        let media = reqwest::multipart::Part::bytes(file_bytes).file_name(file_name);
        let form = reqwest::multipart::Form::new().part("media", media);
        let text = svc
            .http_client()
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &text,
            Some(wx_rust_common::enums::WxType::MiniApp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        serde_json::from_str(&text).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.uploadImg(String, Integer)`：
    /// POST `IMG_UPLOAD?upload_type=1&resp_type=`（`{"img_url": ...}`），
    /// 校验 errcode 后解析 `WxMinishopImageUploadResult`。
    async fn upload_img_from_url(
        &self,
        img_url: &str,
        resp_type: i32,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let base = product_url::other::img_upload_url(config.as_ref());
        let url = format!("{base}?upload_type=1&resp_type={resp_type}");
        let body = build_json(&[("img_url", serde_json::Value::String(img_url.to_string()))]);
        let response = svc.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getCategory`：
    /// 构造 `{"f_cat_id": fCatId}` 后 POST `GET_CATEGORY` 并解析响应。
    async fn get_category(
        &self,
        f_cat_id: Option<i32>,
    ) -> Result<WxMinishopGetCategoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "f_cat_id",
            f_cat_id
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        )]);
        let response = svc
            .post(
                &product_url::other::get_category_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getBrand`：
    /// POST `GET_BRAND`（空对象）并解析响应。
    async fn get_brand(&self) -> Result<WxMinishopGetBrandResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&product_url::other::get_brand_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getFreightTemplate`：
    /// POST `GET_FREIGHT_TEMPLATE`（空对象）并解析响应。
    async fn get_freight_template(
        &self,
    ) -> Result<WxMinishopGetFrightTemplateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &product_url::other::get_freight_template_url(config.as_ref()),
                "{}",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.addSpu`：
    /// POST `PRODUCT_SPU_ADD_URL`（序列化 `WxMinishopSpu`），手工组装
    /// `WxMinishopResult`（data 含 product_id/out_product_id/create_time）。
    async fn add_spu(&self, spu: &WxMinishopSpu) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(spu).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&product_url::spu::add_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let product_id = data
            .get("product_id")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let out_product_id = data
            .get("out_product_id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let create_time = data
            .get("create_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(serde_json::json!({
            "product_id": product_id,
            "out_product_id": out_product_id,
            "create_time": create_time,
        })))
    }

    /// 对应 Java `WxMaProductServiceImpl.deleteSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `PRODUCT_SPU_DEL_URL`。
    async fn delete_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::spu::del_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getSpu`：
    /// 构造 `{"product_id", "out_product_id", "need_edit_spu"}` 后 POST
    /// `PRODUCT_SPU_GET_URL` 并解析响应。
    async fn get_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
        need_edit_spu: Option<i32>,
    ) -> Result<WxMinishopSpuGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "need_edit_spu",
                need_edit_spu
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::spu::get_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getSpuList`：
    /// POST `PRODUCT_SPU_GET_LIST_URL`（序列化 `WxMaShopSpuPageRequest`）后解析响应。
    async fn get_spu_list(
        &self,
        request: &WxMaShopSpuPageRequest,
    ) -> Result<WxMinishopSpuListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&product_url::spu::get_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.updateSpu`：
    /// POST `PRODUCT_SPU_UPDATE_URL`（序列化 `WxMinishopSpu`），手工组装
    /// `WxMinishopResult`（data 含 product_id/out_product_id/update_time）。
    async fn update_spu(&self, spu: &WxMinishopSpu) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(spu).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&product_url::spu::update_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let product_id = data
            .get("product_id")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let out_product_id = data
            .get("out_product_id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let update_time = data
            .get("update_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(serde_json::json!({
            "product_id": product_id,
            "out_product_id": out_product_id,
            "update_time": update_time,
        })))
    }

    /// 对应 Java `WxMaProductServiceImpl.listingSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `PRODUCT_SPU_LISTING_URL`。
    async fn listing_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::spu::listing_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.delistingSpu`：
    /// 构造 `{"product_id", "out_product_id"}` 后 POST `PRODUCT_SPU_DELISTING_URL`。
    async fn delisting_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::spu::delisting_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.getSkuList`：
    /// 构造 `{"product_id", "need_edit_sku", "need_real_stock"}` 后 POST
    /// `PRODUCT_SKU_LIST` 并解析响应。
    async fn get_sku_list(
        &self,
        product_id: i64,
        need_real_stock: Option<i32>,
        need_edit_sku: Option<i32>,
    ) -> Result<WxMinishopSkuListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "need_edit_sku",
                need_edit_sku
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "need_real_stock",
                need_real_stock
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::sku::get_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishiopGoodsAddSku`：
    /// POST `PRODUCT_ADD_SKU_URL`（序列化 `WxMinishopSku`），手工组装
    /// `WxMinishopResult`（data 含 sku_id/create_time）。
    async fn minishop_goods_add_sku(
        &self,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(sku).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&product_url::sku::add_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let sku_id = data
            .get("sku_id")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let create_time = data
            .get("create_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(serde_json::json!({
            "sku_id": sku_id,
            "create_time": create_time,
        })))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishopGoodsBatchAddSku`：
    /// 构造 `{"skus": [...]}` 后 POST `PRODUCT_BATCH_ADD_SKU_URL`，手工组装
    /// `WxMinishopResult`（data 为 `[{sku_id, out_sku_id, create_time}]` 数组）。
    async fn minishop_goods_batch_add_sku(
        &self,
        sku_list: &[WxMinishopSku],
    ) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "skus",
            serde_json::to_value(sku_list).map_err(|e| WxErrorException::Serde(e.to_string()))?,
        )]);
        let response = svc
            .post(&product_url::sku::batch_add_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let items = data
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|element| {
                serde_json::json!({
                    "sku_id": element.get("sku_id").and_then(|v| v.as_i64()).unwrap_or_default(),
                    "out_sku_id": element.get("out_sku_id").and_then(|v| v.as_str()).unwrap_or_default(),
                    "create_time": element.get("create_time").and_then(|v| v.as_str()).unwrap_or_default(),
                })
            })
            .collect::<Vec<_>>();
        Ok(build_result(serde_json::Value::Array(items)))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishopGoodsDelSku`：
    /// 构造 `{"product_id", "out_product_id", "out_sku_id", "sku_id"}` 后 POST
    /// `PRODUCT_DEL_SKU_URL`。
    async fn minishop_goods_del_sku(
        &self,
        product_id: i64,
        out_product_id: Option<i64>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "out_sku_id",
                out_sku_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "sku_id",
                sku_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::sku::del_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishopGoodsUpdateSku`：
    /// POST `PRODUCT_UPDATE_SKU_URL`（序列化 `WxMinishopSku`），手工组装
    /// `WxMinishopResult`（data 含 update_time）。
    async fn minishop_goods_update_sku(
        &self,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(sku).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&product_url::sku::update_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let update_time = data
            .get("update_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(
            serde_json::json!({ "update_time": update_time }),
        ))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishopGoodsUpdateSkuPrice`：
    /// 构造 `{"product_id", "out_product_id", "sku_id", "out_sku_id", "sale_price",
    /// "market_price"}` 后 POST `PRODUCT_UPDATE_SKU_PRICE_URL`。
    async fn minishop_goods_update_sku_price(
        &self,
        product_id: i64,
        out_product_id: Option<&str>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
        sale_price: Option<i64>,
        market_price: Option<i64>,
    ) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "sku_id",
                sku_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "out_sku_id",
                out_sku_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "sale_price",
                sale_price
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "market_price",
                market_price
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::sku::update_price_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let update_time = data
            .get("update_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(
            serde_json::json!({ "update_time": update_time }),
        ))
    }

    /// 对应 Java `WxMaProductServiceImpl.minishopGoodsUpdateSkuStock`：
    /// 构造 `{"product_id", "out_product_id", "sku_id", "out_sku_id", "type",
    /// "stock_num"}` 后 POST `PRODUCT_UPDATE_SKU_STOCK_URL`。
    async fn minishop_goods_update_sku_stock(
        &self,
        product_id: i64,
        out_product_id: Option<&str>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
        r#type: Option<i32>,
        stock_num: Option<i32>,
    ) -> Result<WxMinishopResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("product_id", serde_json::Value::from(product_id)),
            (
                "out_product_id",
                out_product_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "sku_id",
                sku_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "out_sku_id",
                out_sku_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "type",
                r#type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "stock_num",
                stock_num
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::sku::update_stock_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = value
            .get("data")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let update_time = data
            .get("update_time")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(build_result(
            serde_json::json!({ "update_time": update_time }),
        ))
    }
}
