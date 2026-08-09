//! 消息回调事件常量。
//!
//! 对应 Java `me.chanjar.weixin.channel.constant.MessageEventConstants`。

/// 消息回调事件类型常量。
///
/// 对应 Java `MessageEventConstants` 接口的全部字符串常量（品牌资质事件回调、
/// 商品审核、订单、售后、优惠券、资金、团长、会员、分享员、店铺等事件名）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageEventConstants;

impl MessageEventConstants {
    /// 品牌资质事件回调（对应 Java `MessageEventConstants.BRAND`）。
    pub const BRAND: &'static str = "channels_ec_brand";
    /// 商品审核结果（对应 Java `MessageEventConstants.PRODUCT_SPU_AUDIT`）。
    pub const PRODUCT_SPU_AUDIT: &'static str = "product_spu_audit";
    /// 商品上下架（对应 Java `MessageEventConstants.PRODUCT_SPU_STATUS_UPDATE`）。
    pub const PRODUCT_SPU_STATUS_UPDATE: &'static str = "product_spu_listing";
    /// 商品更新（对应 Java `MessageEventConstants.PRODUCT_SPU_UPDATE`）。
    pub const PRODUCT_SPU_UPDATE: &'static str = "product_spu_update";
    /// 类目审核结果（对应 Java `MessageEventConstants.PRODUCT_CATEGORY_AUDIT`）。
    pub const PRODUCT_CATEGORY_AUDIT: &'static str = "product_category_audit";
    /// 库存不足（对应 Java `MessageEventConstants.PRODUCT_STOCK_NO_ENOUGH`）。
    pub const PRODUCT_STOCK_NO_ENOUGH: &'static str = "channels_ec_stock_no_enough";
    /// 订单下单（对应 Java `MessageEventConstants.ORDER_NEW`）。
    pub const ORDER_NEW: &'static str = "channels_ec_order_new";
    /// 订单取消（对应 Java `MessageEventConstants.ORDER_CANCEL`）。
    pub const ORDER_CANCEL: &'static str = "channels_ec_order_cancel";
    /// 订单支付成功（对应 Java `MessageEventConstants.ORDER_PAY`）。
    pub const ORDER_PAY: &'static str = "channels_ec_order_pay";
    /// 订单待发货（对应 Java `MessageEventConstants.ORDER_WAIT_SHIPPING`）。
    pub const ORDER_WAIT_SHIPPING: &'static str = "channels_ec_order_wait_shipping";
    /// 订单发货（对应 Java `MessageEventConstants.ORDER_DELIVER`）。
    pub const ORDER_DELIVER: &'static str = "channels_ec_order_deliver";
    /// 订单确认收货（对应 Java `MessageEventConstants.ORDER_CONFIRM`）。
    pub const ORDER_CONFIRM: &'static str = "channels_ec_order_confirm";
    /// 订单结算成功（对应 Java `MessageEventConstants.ORDER_SETTLE`）。
    pub const ORDER_SETTLE: &'static str = "channels_ec_order_settle";
    /// 订单其他信息更新（对应 Java `MessageEventConstants.ORDER_EXT_INFO_UPDATE`）。
    pub const ORDER_EXT_INFO_UPDATE: &'static str = "channels_ec_order_ext_info_update";
    /// 订单状态更新（对应 Java `MessageEventConstants.ORDER_STATUS_UPDATE`）。
    pub const ORDER_STATUS_UPDATE: &'static str = "product_order_status_update";
    /// 售后单更新通知（对应 Java `MessageEventConstants.AFTER_SALE_UPDATE`）。
    pub const AFTER_SALE_UPDATE: &'static str = "channels_ec_aftersale_update";
    /// 纠纷更新通知（对应 Java `MessageEventConstants.COMPLAINT_NOTIFY`）。
    pub const COMPLAINT_NOTIFY: &'static str = "channels_ec_complaint_update";
    /// 优惠券领取通知（对应 Java `MessageEventConstants.RECEIVE_COUPON`）。
    pub const RECEIVE_COUPON: &'static str = "channels_ec_coupon_receive";
    /// 创建优惠券通知（对应 Java `MessageEventConstants.CREATE_COUPON`）。
    pub const CREATE_COUPON: &'static str = "channels_ec_coupon_create";
    /// 优惠券删除通知（对应 Java `MessageEventConstants.DELETE_COUPON`）。
    pub const DELETE_COUPON: &'static str = "channels_ec_coupon_delete";
    /// 优惠券过期通知（对应 Java `MessageEventConstants.EXPIRE_COUPON`）。
    pub const EXPIRE_COUPON: &'static str = "channels_ec_coupon_expire";
    /// 更新优惠券信息通知（对应 Java `MessageEventConstants.UPDATE_COUPON_INFO`）。
    pub const UPDATE_COUPON_INFO: &'static str = "channels_ec_coupon_info_change";
    /// 优惠券作废通知（对应 Java `MessageEventConstants.INVALID_COUPON`）。
    pub const INVALID_COUPON: &'static str = "channels_ec_coupon_invalid";
    /// 用户优惠券过期通知（对应 Java `MessageEventConstants.USER_COUPON_EXPIRE`）。
    pub const USER_COUPON_EXPIRE: &'static str = "channels_ec_user_coupon_expire";
    /// 优惠券返还通知（对应 Java `MessageEventConstants.USER_COUPON_UNUSE`）。
    pub const USER_COUPON_UNUSE: &'static str = "channels_ec_user_coupon_unuse";
    /// 优惠券核销通知（对应 Java `MessageEventConstants.USER_COUPON_USE`）。
    pub const USER_COUPON_USE: &'static str = "channels_ec_user_coupon_use";
    /// 发放团购优惠成功回调（对应 Java `MessageEventConstants.VOUCHER_SEND_SUCC`）。
    pub const VOUCHER_SEND_SUCC: &'static str = "channels_ec_voucher_send_succ";
    /// 结算账户变更回调（对应 Java `MessageEventConstants.ACCOUNT_NOTIFY`）。
    pub const ACCOUNT_NOTIFY: &'static str = "channels_ec_acct_notify";
    /// 提现回调（对应 Java `MessageEventConstants.WITHDRAW_NOTIFY`）。
    pub const WITHDRAW_NOTIFY: &'static str = "channels_ec_withdraw_notify";
    /// 提现二维码回调（对应 Java `MessageEventConstants.QRCODE_STATUS`）。
    pub const QRCODE_STATUS: &'static str = "qrcode_status";
    /// 团长合作商品更新（对应 Java `MessageEventConstants.SUPPLIER_ITEM_UPDATE`）。
    pub const SUPPLIER_ITEM_UPDATE: &'static str = "head_supplier_item_update";
    /// 进入会话事件（对应 Java `MessageEventConstants.USER_ENTER_TEMP_SESSION`）。
    pub const USER_ENTER_TEMP_SESSION: &'static str = "user_enter_tempsession";
    /// 用户加入会员（对应 Java `MessageEventConstants.USER_VIP_JOIN`）。
    pub const USER_VIP_JOIN: &'static str = "channels_ec_vip_join";
    /// 用户注销会员（对应 Java `MessageEventConstants.USER_VIP_CLOSE`）。
    pub const USER_VIP_CLOSE: &'static str = "channels_ec_vip_close";
    /// 用户等级更新（对应 Java `MessageEventConstants.USER_VIP_GRADE_INFO_UPDATE`）。
    pub const USER_VIP_GRADE_INFO_UPDATE: &'static str = "channels_ec_vip_grade_info_update";
    /// 用户积分更新（对应 Java `MessageEventConstants.USER_VIP_SCORE_UPDATE`）。
    pub const USER_VIP_SCORE_UPDATE: &'static str = "channels_ec_vip_score_update";
    /// 用户积分兑换（对应 Java `MessageEventConstants.USER_VIP_SCORE_EXCHANGE`）。
    pub const USER_VIP_SCORE_EXCHANGE: &'static str = "channels_ec_vip_score_exchange";
    /// 分享员变更（对应 Java `MessageEventConstants.SHARER_CHANGE`）。
    pub const SHARER_CHANGE: &'static str = "channels_ec_sharer_change";
    /// 小店注销（对应 Java `MessageEventConstants.CLOSE_STORE`）。
    pub const CLOSE_STORE: &'static str = "channels_ec_close_store";
    /// 小店修改（对应 Java `MessageEventConstants.SET_SHOP_NICKNAME`）。
    pub const SET_SHOP_NICKNAME: &'static str = "set_shop_nickname";
}
