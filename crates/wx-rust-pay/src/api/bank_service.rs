//! 对应 Java `com.github.binarywang.wxpay.service.BankService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// BankService（对应 Java `BankService`）。
#[async_trait]
pub trait BankService: Send + Sync {
    /// 微信支付-银行组件
    async fn search_banks_by_bank_account(
        &self,
        account_number: &str,
    ) -> Result<BankAccountResult, WxErrorException>;

    /// 查询支持个人业务的银行列表 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com/v3/capital/capitallhh/banks/persona
    async fn personal_banking(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<BankingResult, WxErrorException>;

    /// 支持对公业务的银行列表 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com/v3/capital/capitallhh/banks/corporate
    async fn corporate_banking(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<BankingResult, WxErrorException>;

    /// 查询省份列表API 通过本接口获取省份列表数据（不包含中国港澳台地区），可用于省份下的城市数据查询 请求方式：GET（HTTPS） 请求地址：https://api.mch.weixin.qq.com
    async fn areas_provinces(&self) -> Result<ProvincesResult, WxErrorException>;

    /// 查询城市列表API 通过本接口根据省份编码获取省份下的城市列表信息，不包含中国港澳台地区城市信息，可用于支行数据过滤查询 请求方式：GET（HTTPS） 请求地址：https://api.mch.we
    async fn areas_cities(&self, province_code: i32) -> Result<CitiesResult, WxErrorException>;

    /// 查询支行列表API 本接口可以用于根据银行别名编码（仅支持需要填写支行的银行别名编码）和城市编码过滤查询支行列表数据 请求方式：GET（HTTPS） 请求地址：https://api.mch.weix
    async fn bank_branches(
        &self,
        bank_alias_code: &str,
        city_code: i32,
        offset: i32,
        limit: i32,
    ) -> Result<BankBranchesResult, WxErrorException>;
}
