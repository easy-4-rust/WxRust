//! Wave 2 G3 组（电商子服务）接口地址。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaApiUrlConstants` 中
//! shop/product/orderManagement/orderShipping/instantDelivery/employee/
//! expressDeliveryReturn/customservice 各子域地址。函数风格与 `url_business`
//! 一致：config 参数 + api_host 前缀模式（自定义域名替换由执行引擎在 token
//! 注入时统一处理）。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 小程序交易组件-商家入驻接口地址（对应 Java `WxMaApiUrlConstants.Shop.Account`）。
pub mod shop_account {
    use super::*;

    /// 获取商家类目列表（对应 Java `Account.GET_CATEGORY_LIST`）。
    pub fn get_category_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/account/get_category_list")
    }

    /// 获取商家品牌列表（对应 Java `Account.GET_BRAND_LIST`）。
    pub fn get_brand_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/account/get_brand_list")
    }

    /// 更新商家信息（对应 Java `Account.UPDATE_INFO`）。
    pub fn update_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/account/update_info")
    }

    /// 获取商家信息（对应 Java `Account.GET_INFO`）。
    pub fn get_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/account/get_info")
    }
}

/// 小程序交易组件-售后服务地址（对应 Java `WxMaApiUrlConstants.Shop.Aftersale`）。
pub mod shop_aftersale {
    use super::*;

    /// 创建售后（对应 Java `Aftersale.AFTERSALE_ADD`）。
    pub fn add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/add")
    }

    /// 用户取消售后申请（对应 Java `Aftersale.AFTERSALE_CANCEL`）。
    pub fn cancel_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/cancel")
    }

    /// 更新售后（对应 Java `Aftersale.AFTERSALE_UPDATE`）。
    pub fn update_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/aftersale/update")
    }

    /// 更新售后（EC 版，对应 Java `Aftersale.EC_AFTERSALE_UPDATE`）。
    pub fn ec_update_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/update")
    }

    /// 用户上传退货物流（对应 Java `Aftersale.AFTERSALE_UPLOAD_RETURN_INFO`）。
    pub fn upload_return_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/uploadreturninfo")
    }

    /// 商家同意退款（对应 Java `Aftersale.AFTERSALE_ACCEPT_REFUND`）。
    pub fn accept_refund_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/acceptrefund")
    }

    /// 商家同意退货（对应 Java `Aftersale.AFTERSALE_ACCEPT_RETURN`）。
    pub fn accept_return_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/acceptreturn")
    }

    /// 商家拒绝售后（对应 Java `Aftersale.AFTERSALE_REJECT`）。
    pub fn reject_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/reject")
    }

    /// 商家上传退款凭证（对应 Java `Aftersale.AFTERSALE_UPLOAD_CERTIFICATES`）。
    pub fn upload_certificates_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/upload_certificates")
    }

    /// 商家更新订单售后期（对应 Java `Aftersale.AFTERSALE_UPLOAD_DEADLINE`）。
    pub fn update_deadline_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/aftersale/update_deadline")
    }

    /// 获取售后单列表（对应 Java `Aftersale.AFTERSALE_GET_LIST`）。
    pub fn get_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/get_list")
    }

    /// 获取订单下售后单（对应 Java `Aftersale.AFTERSALE_GET`）。
    pub fn get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/aftersale/get")
    }

    /// 获取售后单详情（EC 版，对应 Java `Aftersale.ECAFTERSALE_GET`）。
    pub fn ec_get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/ecaftersale/get")
    }
}

/// 小程序交易组件-审核相关接口地址（对应 Java `WxMaApiUrlConstants.Shop.Audit`）。
pub mod shop_audit {
    use super::*;

    /// 上传品牌信息（品牌审核，对应 Java `Audit.AUDIT_BRAND`）。
    pub fn audit_brand_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/audit/audit_brand")
    }

    /// 上传类目资质（类目审核，对应 Java `Audit.AUDIT_CATEGORY`）。
    pub fn audit_category_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/audit/audit_category")
    }

    /// 获取审核结果（对应 Java `Audit.AUDIT_RESULT`）。
    pub fn audit_result_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/audit/result")
    }

    /// 获取小程序提交过的入驻资质信息（对应 Java `Audit.GET_MINIAPP_CERTIFICATE`）。
    pub fn get_miniapp_certificate_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/audit/get_miniapp_certificate")
    }
}

/// 小程序交易组件-商品类目地址（对应 Java `WxMaApiUrlConstants.Shop.Cat`）。
pub mod shop_cat {
    use super::*;

    /// 获取商品类目（对应 Java `Cat.GET_CAT`）。
    pub fn get_cat_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/cat/get")
    }
}

/// 小程序交易组件-优惠券地址（对应 Java `WxMaApiUrlConstants.Shop.Coupon`）。
pub mod shop_coupon {
    use super::*;

    /// 添加优惠券（对应 Java `Coupon.ADD_COUPON`）。
    pub fn add_coupon_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/add")
    }

    /// 获取优惠券（对应 Java `Coupon.GET_COUPON`）。
    pub fn get_coupon_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/get")
    }

    /// 获取优惠券列表（对应 Java `Coupon.GET_COUPON_LIST`）。
    pub fn get_coupon_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/get_list")
    }

    /// 更新优惠券（对应 Java `Coupon.UPDATE_COUPON`）。
    pub fn update_coupon_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/update")
    }

    /// 更新优惠券状态（对应 Java `Coupon.UPDATE_COUPON_STATUS`）。
    pub fn update_coupon_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/update_status")
    }

    /// 更新优惠券库存（对应 Java `Coupon.UPDATE_COUPON_STOCK`）。
    pub fn update_coupon_stock_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/update_coupon_stock")
    }

    /// 添加用户优惠券（对应 Java `Coupon.ADD_USER_COUPON`）。
    pub fn add_user_coupon_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/add_user_coupon")
    }

    /// 获取用户优惠券列表（对应 Java `Coupon.GET_USER_COUPON_LIST`）。
    pub fn get_user_coupon_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/get_usercoupon_list")
    }

    /// 更新用户优惠券（对应 Java `Coupon.UPDATE_USER_COUPON`）。
    pub fn update_user_coupon_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/update_user_coupon")
    }

    /// 更新用户优惠券状态（对应 Java `Coupon.UPDATE_USER_COUPON_STATUS`）。
    pub fn update_user_coupon_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/coupon/update_usercoupon_status")
    }
}

/// 小程序交易组件-物流发货服务地址（对应 Java `WxMaApiUrlConstants.Shop.Delivery`）。
pub mod shop_delivery {
    use super::*;

    /// 获取快递公司列表（对应 Java `Delivery.GET_COMPANY_LIST`）。
    pub fn get_company_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/delivery/get_company_list")
    }

    /// 订单发货（对应 Java `Delivery.DELIVERY_SEND`）。
    pub fn delivery_send_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/delivery/send")
    }

    /// 订单确认收货（对应 Java `Delivery.DELIVERY_RECEIVE`，原常量拼写为 recieve）。
    pub fn delivery_receive_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/delivery/recieve")
    }
}

/// 小程序交易组件-图片上传地址（对应 Java `WxMaApiUrlConstants.Shop.Img`）。
pub mod shop_img {
    use super::*;

    /// 上传图片（对应 Java `Img.IMG_UPLOAD`）。
    pub fn img_upload_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/img/upload")
    }
}

/// 小程序交易组件-订单服务地址（对应 Java `WxMaApiUrlConstants.Shop.Order`）。
pub mod shop_order {
    use super::*;

    /// 场景检查（对应 Java `Order.ORDER_CHECK_SCENE`）。
    pub fn check_scene_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/scene/check")
    }

    /// 添加订单（对应 Java `Order.ORDER_ADD`）。
    pub fn order_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/order/add")
    }

    /// 订单支付（对应 Java `Order.ORDER_PAY`）。
    pub fn order_pay_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/order/pay")
    }

    /// 获取订单（对应 Java `Order.ORDER_GET`）。
    pub fn order_get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/order/get")
    }

    /// 获取订单列表（对应 Java `Order.ORDER_GET_LIST`）。
    pub fn order_get_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/order/get_list")
    }

    /// 生成支付参数（对应 Java `Order.ORDER_GET_PAYMENT_PARAMS`）。
    pub fn order_get_payment_params_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/order/getpaymentparams")
    }
}

/// 小程序支付管理订单相关接口地址（对应 Java `WxMaApiUrlConstants.Shop.Pay`）。
pub mod shop_pay {
    use super::*;

    /// 创建订单（对应 Java `Pay.CREATE_ORDER`）。
    pub fn create_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/pay/createorder")
    }

    /// 查询订单详情（对应 Java `Pay.GET_ORDER`）。
    pub fn get_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/pay/getorder")
    }

    /// 订单退款（对应 Java `Pay.REFUND_ORDER`）。
    pub fn refund_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/pay/refundorder")
    }
}

/// 小程序交易组件-申请接入服务地址（对应 Java `WxMaApiUrlConstants.Shop.Register`）。
pub mod shop_register {
    use super::*;

    /// 接入申请（对应 Java `Register.REGISTER_APPLY`）。
    pub fn register_apply_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/register/apply")
    }

    /// 获取接入状态（对应 Java `Register.REGISTER_CHECK`）。
    pub fn register_check_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/register/check")
    }

    /// 完成接入任务（对应 Java `Register.REGISTER_FINISH_ACCESS_INFO`）。
    pub fn register_finish_access_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/register/finish_access_info")
    }

    /// 场景接入申请（对应 Java `Register.REGISTER_APPLY_SCENE`）。
    pub fn register_apply_scene_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/register/apply_scene")
    }
}

/// 小程序交易组件-分享员地址（对应 Java `WxMaApiUrlConstants.Shop.Sharer`）。
pub mod shop_sharer {
    use super::*;

    /// 绑定分享员（对应 Java `Sharer.BIND`）。
    pub fn bind_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/sharer/bind")
    }

    /// 获取分享员的总带货数据（对应 Java `Sharer.GET_SHARER_DATA_SUMMARY`）。
    pub fn get_sharer_data_summary_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/sharer/get_sharer_data_summary")
    }

    /// 获取已经绑定的分享员列表（对应 Java `Sharer.GET_SHARER_LIST`）。
    pub fn get_sharer_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/sharer/get_sharer_list")
    }

    /// 获取分享员的直播间订单汇总（对应 Java `Sharer.GET_SHARER_LIVE_ORDER_LIST`）。
    pub fn get_sharer_live_order_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/shop/sharer/get_sharer_live_order_list",
        )
    }

    /// 获取分享员的直播间带货数据汇总（对应 Java `Sharer.GET_SHARER_LIVE_SUMMARY_LIST`）。
    pub fn get_sharer_live_summary_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/shop/sharer/get_sharer_live_summary_list",
        )
    }

    /// 查看分享员（对应 Java `Sharer.SEARCH_SHARER`）。
    pub fn search_sharer_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/sharer/search_sharer")
    }

    /// 解绑分享员（对应 Java `Sharer.UNBIND`）。
    pub fn unbind_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/sharer/unbind")
    }
}

/// 小程序交易组件-商品服务地址（对应 Java `WxMaApiUrlConstants.Shop.Spu`）。
pub mod shop_spu {
    use super::*;

    /// 添加商品（对应 Java `Spu.SPU_ADD_URL`）。
    pub fn spu_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/add")
    }

    /// 删除商品（对应 Java `Spu.SPU_DEL_URL`）。
    pub fn spu_del_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/del")
    }

    /// 获取商品（对应 Java `Spu.SPU_GET_URL`）。
    pub fn spu_get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/get")
    }

    /// 获取商品列表（对应 Java `Spu.SPU_GET_LIST_URL`）。
    pub fn spu_get_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/get_list")
    }

    /// 更新商品（对应 Java `Spu.SPU_UPDATE_URL`）。
    pub fn spu_update_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/update")
    }

    /// 免审核更新商品（对应 Java `Spu.SPU_UPDATE_WITHOUT_URL`）。
    pub fn spu_update_without_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/update_without_audit")
    }

    /// 商品上架（对应 Java `Spu.SPU_LISTING_URL`）。
    pub fn spu_listing_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/listing")
    }

    /// 商品下架（对应 Java `Spu.SPU_DELISTING_URL`）。
    pub fn spu_delisting_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/delisting")
    }

    /// 撤回审核（对应 Java `Spu.DEL_AUDIT_URL`）。
    pub fn spu_del_audit_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/shop/spu/del_audit")
    }
}

/// 小程序交易组件-标准版商品服务地址（对应 Java `WxMaApiUrlConstants.Product`）。
pub mod product {
    use super::*;

    /// 标准版商品 SPU 地址（对应 Java `Product.Spu`）。
    pub mod spu {
        use super::*;

        /// 添加商品（对应 Java `Spu.PRODUCT_SPU_ADD_URL`）。
        pub fn add_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/add")
        }

        /// 删除商品（对应 Java `Spu.PRODUCT_SPU_DEL_URL`）。
        pub fn del_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/del")
        }

        /// 获取商品（对应 Java `Spu.PRODUCT_SPU_GET_URL`）。
        pub fn get_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/get")
        }

        /// 获取商品列表（对应 Java `Spu.PRODUCT_SPU_GET_LIST_URL`）。
        pub fn get_list_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/get_list")
        }

        /// 更新商品（对应 Java `Spu.PRODUCT_SPU_UPDATE_URL`）。
        pub fn update_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/update")
        }

        /// 商品上架（对应 Java `Spu.PRODUCT_SPU_LISTING_URL`）。
        pub fn listing_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/listing")
        }

        /// 商品下架（对应 Java `Spu.PRODUCT_SPU_DELISTING_URL`）。
        pub fn delisting_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/spu/delisting")
        }
    }

    /// 标准版商品 SKU 地址（对应 Java `Product.Sku`）。
    pub mod sku {
        use super::*;

        /// 添加 SKU（对应 Java `Sku.PRODUCT_ADD_SKU_URL`）。
        pub fn add_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/add")
        }

        /// 批量添加 SKU（对应 Java `Sku.PRODUCT_BATCH_ADD_SKU_URL`）。
        pub fn batch_add_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/batch_add")
        }

        /// 删除 SKU（对应 Java `Sku.PRODUCT_DEL_SKU_URL`）。
        pub fn del_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/del")
        }

        /// 更新 SKU（对应 Java `Sku.PRODUCT_UPDATE_SKU_URL`）。
        pub fn update_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/update")
        }

        /// 更新 SKU 价格（对应 Java `Sku.PRODUCT_UPDATE_SKU_PRICE_URL`）。
        pub fn update_price_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/update_price")
        }

        /// 更新 SKU 库存（对应 Java `Sku.PRODUCT_UPDATE_SKU_STOCK_URL`）。
        pub fn update_stock_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/stock/update")
        }

        /// 获取 SKU 列表（对应 Java `Sku.PRODUCT_SKU_LIST`）。
        pub fn get_list_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/sku/get_list")
        }
    }

    /// 标准版商品订单地址（对应 Java `Product.Order`）。
    pub mod order {
        use super::*;

        /// 获取订单列表（对应 Java `Order.PRODUCT_ORDER_GET_LIST`）。
        pub fn get_list_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/get_list")
        }

        /// 获取订单详情（对应 Java `Order.PRODUCT_ORDER_DETAIL_URL`）。
        pub fn detail_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/get")
        }

        /// 修改订单备注（对应 Java `Order.PRODUCT_ORDER_CHANGE_MERCHANT_NOTES_URL`）。
        pub fn change_merchant_notes_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/change_merchant_notes")
        }

        /// 订单发货（对应 Java `Order.PRODUCT_DELIVERY_SEND`）。
        pub fn delivery_send_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/delivery/send")
        }

        /// 获取售后单（对应 Java `Order.GET_AFTER_SALE_ORDER`）。
        pub fn get_after_sale_order_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/getaftersaleorder")
        }

        /// 批量获取售后单（对应 Java `Order.BATCH_GET_AFTER_SALE_ORDER`）。
        pub fn batch_get_after_sale_order_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/batchgetaftersaleorder")
        }

        /// 同意售后申请（对应 Java `Order.AFTER_SALE_ACCEPT_APPLY`）。
        pub fn after_sale_accept_apply_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/acceptapply")
        }

        /// 拒绝售后申请（对应 Java `Order.AFTER_SALE_REJECT_APPLY`）。
        pub fn after_sale_reject_apply_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/order/rejectrefund")
        }
    }

    /// 标准版商品其他地址（对应 Java `Product.OTHER`）。
    pub mod other {
        use super::*;

        /// 获取商品类目（对应 Java `OTHER.GET_CATEGORY`）。
        pub fn get_category_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/category/get")
        }

        /// 获取品牌列表（对应 Java `OTHER.GET_BRAND`）。
        pub fn get_brand_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/brand/get")
        }

        /// 获取运费模板（对应 Java `OTHER.GET_FREIGHT_TEMPLATE`）。
        pub fn get_freight_template_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/product/delivery/get_freight_template",
            )
        }

        /// 上传图片（对应 Java `OTHER.IMG_UPLOAD`）。
        pub fn img_upload_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/product/img/upload")
        }
    }
}

/// 小程序订单管理接口地址（对应 Java `WxMaApiUrlConstants.OrderManagement`）。
pub mod order_management {
    use super::*;

    /// 查询订单详情路径（对应 Java `OrderManagement.GET_ORDER_DETAIL_PATH`）。
    pub fn get_order_detail_path_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/get_order_detail_path")
    }

    /// 配置订单详情路径（对应 Java `OrderManagement.UPDATE_ORDER_DETAIL_PATH`）。
    pub fn update_order_detail_path_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxa/sec/order/update_order_detail_path",
        )
    }
}

/// 发货信息管理服务接口地址（对应 Java `WxMaApiUrlConstants.OrderShipping`）。
pub mod order_shipping {
    use super::*;

    /// 查询小程序是否已开通发货信息管理服务（对应 Java `OrderShipping.IS_TRADE_MANAGED`）。
    pub fn is_trade_managed_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/is_trade_managed")
    }

    /// 发货信息录入（对应 Java `OrderShipping.UPLOAD_SHIPPING_INFO`）。
    pub fn upload_shipping_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/upload_shipping_info")
    }

    /// 发货信息合单录入（对应 Java `OrderShipping.UPLOAD_COMBINED_SHIPPING_INFO`）。
    pub fn upload_combined_shipping_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxa/sec/order/upload_combined_shipping_info",
        )
    }

    /// 查询订单发货状态（对应 Java `OrderShipping.GET_SHIPPING_INFO`）。
    pub fn get_shipping_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/get_order")
    }

    /// 查询订单发货状态列表（对应 Java `OrderShipping.GET_SHIPPING_INFO_LIST`）。
    pub fn get_shipping_info_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/get_order_list")
    }

    /// 确认收货提醒（对应 Java `OrderShipping.NOTIFY_CONFIRM_RECEIVE`）。
    pub fn notify_confirm_receive_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/notify_confirm_receive")
    }

    /// 消息跳转路径设置（对应 Java `OrderShipping.SET_MSG_JUMP_PATH`）。
    pub fn set_msg_jump_path_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/set_msg_jump_path")
    }

    /// 查询小程序是否已完成交易结算管理确认（对应 Java
    /// `OrderShipping.IS_TRADE_MANAGEMENT_CONFIRMATION_COMPLETED`）。
    pub fn is_trade_management_confirmation_completed_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxa/sec/order/is_trade_management_confirmation_completed",
        )
    }

    /// 特殊发货报备（对应 Java `OrderShipping.OP_SPECIAL_ORDER`）。
    pub fn op_special_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/order/opspecialorder")
    }
}

/// 微信小程序物流退货组件接口地址（对应 Java `WxMaApiUrlConstants.ExpressDeliveryReturn`）。
pub mod express_delivery_return {
    use super::*;

    /// 新增退货单（对应 Java `ExpressDeliveryReturn.ADD_DELIVERY_RETURN_URL`）。
    pub fn add_delivery_return_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/delivery/return/add")
    }

    /// 获取退货单（对应 Java `ExpressDeliveryReturn.GET_DELIVERY_RETURN_URL`）。
    pub fn get_delivery_return_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/delivery/return/get")
    }

    /// 取消退货单（对应 Java `ExpressDeliveryReturn.UNBIND_DELIVERY_RETURN_URL`）。
    pub fn unbind_delivery_return_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/return/unbind",
        )
    }
}

/// 微信小程序即时配送服务接口地址（对应 Java `WxMaApiUrlConstants.InstantDelivery`）。
pub mod instant_delivery {
    use super::*;

    /// 拉取已绑定账号（对应 Java `InstantDelivery.GET_BIND_ACCOUNT`）。
    pub fn get_bind_account_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/shop/get",
        )
    }

    /// 拉取配送单信息（对应 Java `InstantDelivery.GET_ORDER`）。
    pub fn get_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/get",
        )
    }

    /// 模拟配送公司更新配送单状态（对应 Java `InstantDelivery.MOCK_UPDATE_ORDER`）。
    pub fn mock_update_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/test_update_order",
        )
    }

    /// 跟踪物流面单（对应 Java `InstantDelivery.TRACE_WAYBILL_URL`）。
    pub fn trace_waybill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/trace_waybill",
        )
    }

    /// 查询运单详情（对应 Java `InstantDelivery.QUERY_WAYBILL_TRACE_URL`）。
    pub fn query_waybill_trace_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/query_trace",
        )
    }

    /// 传运单（订阅消息，对应 Java `InstantDelivery.FOLLOW_WAYBILL_URL`）。
    pub fn follow_waybill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/follow_waybill",
        )
    }

    /// 查运单（订阅消息，对应 Java `InstantDelivery.QUERY_FOLLOW_TRACE_URL`）。
    pub fn query_follow_trace_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/query_follow_trace",
        )
    }

    /// 获取运力 id 列表（对应 Java `InstantDelivery.GET_DELIVERY_LIST_URL`）。
    pub fn get_delivery_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/get_delivery_list",
        )
    }

    /// 更新物品信息（对应 Java `InstantDelivery.UPDATE_WAYBILL_GOODS_URL`）。
    pub fn update_waybill_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/delivery/open_msg/update_waybill_goods",
        )
    }

    /// 下配送单（对应 Java `InstantDelivery.PlaceAnOrder.ADD_ORDER`）。
    pub fn add_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/add",
        )
    }

    /// 取消配送单（对应 Java `InstantDelivery.Cancel.CANCEL_ORDER`）。
    pub fn cancel_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/cancel",
        )
    }

    /// 异常件退回商家确认收货（对应 Java `InstantDelivery.Cancel.ABNORMAL_CONFIRM`）。
    pub fn abnormal_confirm_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/confirm_return",
        )
    }

    /// 获取已支持的配送公司列表（对应 Java `InstantDelivery.PlaceAnOrder.GET_ALL_IMME_DELIVERY`）。
    pub fn get_all_imme_delivery_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/delivery/getall",
        )
    }

    /// 预下配送单（对应 Java `InstantDelivery.PlaceAnOrder.PRE_ADD_ORDER`）。
    pub fn pre_add_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/pre_add",
        )
    }

    /// 重新下单（对应 Java `InstantDelivery.PlaceAnOrder.RE_ORDER`）。
    pub fn re_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/readd",
        )
    }

    /// 增加小费（对应 Java `InstantDelivery.PlaceAnOrder.ADD_TIP`）。
    pub fn add_tip_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/addtips",
        )
    }

    /// 预取消配送单（对应 Java `InstantDelivery.Cancel.PRE_CANCEL_ORDER`）。
    pub fn pre_cancel_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/local/business/order/precancel",
        )
    }
}

/// 小程序用工关系接口地址（对应 Java `WxMaApiUrlConstants.Employee`）。
pub mod employee {
    use super::*;

    /// 解绑用工关系（对应 Java `Employee.UNBIND_EMPLOYEE_URL`）。
    pub fn unbind_employee_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/unbinduserb2cauthinfo")
    }

    /// 推送用工消息（对应 Java `Employee.SEND_EMPLOYEE_MSG_URL`）。
    pub fn send_employee_msg_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/message/wxopen/employeerelationmsg/send",
        )
    }
}

/// 小程序微信客服绑定接口地址（对应 Java `WxMaCustomserviceWorkService` 常量）。
pub mod customservice_work {
    use super::*;

    /// 查询小程序的微信客服绑定情况（对应 Java `GET_CUSTOMSERVICE_URL`）。
    pub fn get_customservice_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/work/get")
    }

    /// 为小程序绑定微信客服（对应 Java `BIND_CUSTOMSERVICE_URL`）。
    pub fn bind_customservice_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/work/bind")
    }

    /// 为小程序解除绑定微信客服（对应 Java `UNBIND_CUSTOMSERVICE_URL`）。
    pub fn unbind_customservice_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/customservice/work/unbind")
    }
}

/// 默认 API 域名字面量（与 `url_core::API_HOST` 一致，供本模块内部使用）。
#[allow(unused)]
const API_HOST: &str = DEFAULT_API_HOST_URL;
