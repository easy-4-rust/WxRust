//! 消息服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.BaseWxChannelMessageServiceImpl`
//! （Wave 2 H2c 迁移）：持有消息路由器，构造时注册全部默认回调规则
//! （Java `addDefaultRule`，39 条：品牌/商品/订单/售后/优惠券/资金/会员/
//! 分享员/店铺/团长/团购券），每条规则的 handler 返回 `"success"`
//! （对应 Java `consumer.accept(...); return "success";`）。
//!
//! Java 为抽象基类（子类覆写事件方法扩展业务）；Rust 无继承，扩展点为
//! 路由器规则自定义（`router_mut()` 替换/追加规则、`add_rule` 注册自定义
//! 消费者），默认事件方法即 Java 基类的 log-only 行为（本仓库无日志依赖，
//! 空实现，见 [`crate::api::WxChannelMessageService`] 文档，ADAPTED）。
//!
//! 注：本文件位于 `api/impl/` 目录，由 `impl/mod.rs` 统一注册为
//! `crate::api::r#impl::wx_channel_message_service_impl`（Wave 3 收尾，
//! 原 `api/mod.rs` 的 `#[path]` 临时注册已移除，`api/mod.rs` 仅重导出）。

use std::sync::Arc;

use async_trait::async_trait;

use crate::api::WxChannelService;
use crate::api::wx_channel_message_service::WxChannelMessageService;
use crate::bean::message::after::{AfterSaleMessage, ComplaintMessage};
use crate::bean::message::coupon::{
    CouponActionMessage, CouponReceiveMessage, UserCouponExpireMessage,
};
use crate::bean::message::fund::{AccountNotifyMessage, QrNotifyMessage, WithdrawNotifyMessage};
use crate::bean::message::order::{
    OrderCancelMessage, OrderConfirmMessage, OrderDeliveryMessage, OrderExtMessage, OrderIdMessage,
    OrderPayMessage, OrderSettleMessage, OrderStatusMessage,
};
use crate::bean::message::product::{
    BrandMessage, CategoryAuditMessage, SpuAuditMessage, SpuStockMessage,
};
use crate::bean::message::sharer::SharerChangeMessage;
use crate::bean::message::store::{CloseStoreMessage, NicknameUpdateMessage};
use crate::bean::message::supplier::SupplierItemMessage;
use crate::bean::message::vip::{ExchangeInfoMessage, UserInfoMessage};
use crate::bean::message::voucher::VoucherMessage;
use crate::constant::MessageEventConstants;
use crate::message::rule::{HandlerConsumer, WxChannelMessageHandlerFn};
use crate::message::{
    WxChannelMessage, WxChannelMessageLike, WxChannelMessageRouter, WxChannelMessageRouterRule,
    WxChannelMessageRouterRuleErased,
};

/// 消息服务实现（对应 Java `BaseWxChannelMessageServiceImpl`）。
pub struct WxChannelMessageServiceImpl {
    /// 消息路由器（对应 Java `router`）。
    router: WxChannelMessageRouter,
}

impl WxChannelMessageServiceImpl {
    /// 新建消息服务：内部创建路由器并注册默认回调规则
    /// （对应 Java `BaseWxChannelMessageServiceImpl(WxChannelMessageRouter)`）。
    pub fn new() -> Self {
        let mut service = Self {
            router: WxChannelMessageRouter::new(),
        };
        service.add_default_rule();
        service
    }

    /// 使用自定义路由器构建消息服务（对应 Java 构造器注入 router 后
    /// `addDefaultRule`）。
    pub fn with_router(router: WxChannelMessageRouter) -> Self {
        let mut service = Self { router };
        service.add_default_rule();
        service
    }

    /// 返回路由器（只读）。
    pub fn router(&self) -> &WxChannelMessageRouter {
        &self.router
    }

    /// 返回路由器（可变，用于规则自定义扩展）。
    pub fn router_mut(&mut self) -> &mut WxChannelMessageRouter {
        &mut self.router
    }

    /// 添加一条规则进入路由器（对应 Java 受保护的
    /// `addRule(Class<T>, String, Boolean, HandlerConsumer)`）。
    ///
    /// 规则行为：设置事件（msgType 自动置为 "event"）+ 异步开关 + 处理器
    /// 包装消费者并返回 `"success"` + `next(true)`（Java 同款）。
    pub fn add_rule<T: WxChannelMessageLike>(
        &mut self,
        event: &str,
        async_exec: bool,
        consumer: HandlerConsumer<T>,
    ) {
        let handler = WxChannelMessageHandlerFn::new(
            move |message, content, app_id, context, session_manager| {
                // Java: consumer.accept(message, content, appId, context, sessionManager);
                consumer(message, content, app_id, context, session_manager);
                // Java: return "success";
                Ok(Some("success".to_string()))
            },
        );
        let mut rule = WxChannelMessageRouterRule::<T>::new();
        rule.set_event(event);
        rule.async_exec = async_exec;
        rule.handlers.push(Arc::new(handler));
        rule.next = true;
        self.router.add_rule(Arc::new(rule));
    }

    /// 添加默认的回调规则（对应 Java `addDefaultRule`）。
    fn add_default_rule(&mut self) {
        /* 品牌资质事件回调 this::brandUpdate */
        self.add_rule::<BrandMessage>(MessageEventConstants::BRAND, true, noop_consumer());
        /* 商品审核结果 this::spuAudit */
        self.add_rule::<SpuAuditMessage>(
            MessageEventConstants::PRODUCT_SPU_AUDIT,
            true,
            noop_consumer(),
        );
        /* 商品上下架 this::spuStatusUpdate */
        self.add_rule::<SpuAuditMessage>(
            MessageEventConstants::PRODUCT_SPU_STATUS_UPDATE,
            true,
            noop_consumer(),
        );
        /* 商品更新 this::spuUpdate */
        self.add_rule::<SpuAuditMessage>(
            MessageEventConstants::PRODUCT_SPU_UPDATE,
            true,
            noop_consumer(),
        );
        /* 商品库存不足 this::stockNoEnough */
        self.add_rule::<SpuStockMessage>(
            MessageEventConstants::PRODUCT_STOCK_NO_ENOUGH,
            true,
            noop_consumer(),
        );
        /* 类目审核结果 this::categoryAudit */
        self.add_rule::<CategoryAuditMessage>(
            MessageEventConstants::PRODUCT_CATEGORY_AUDIT,
            true,
            noop_consumer(),
        );
        /* 订单下单 this::orderNew */
        self.add_rule::<OrderIdMessage>(MessageEventConstants::ORDER_NEW, true, noop_consumer());
        /* 订单取消 this::orderCancel */
        self.add_rule::<OrderCancelMessage>(
            MessageEventConstants::ORDER_CANCEL,
            true,
            noop_consumer(),
        );
        /* 订单支付成功 this::orderPay */
        self.add_rule::<OrderPayMessage>(MessageEventConstants::ORDER_PAY, true, noop_consumer());
        /* 订单待发货 this::orderWaitShipping */
        self.add_rule::<OrderIdMessage>(
            MessageEventConstants::ORDER_WAIT_SHIPPING,
            true,
            noop_consumer(),
        );
        /* 订单发货 this::orderDelivery */
        self.add_rule::<OrderDeliveryMessage>(
            MessageEventConstants::ORDER_DELIVER,
            true,
            noop_consumer(),
        );
        /* 订单确认收货 this::orderConfirm */
        self.add_rule::<OrderConfirmMessage>(
            MessageEventConstants::ORDER_CONFIRM,
            true,
            noop_consumer(),
        );
        /* 订单结算成功 this::orderSettle */
        self.add_rule::<OrderSettleMessage>(
            MessageEventConstants::ORDER_SETTLE,
            true,
            noop_consumer(),
        );
        /* 订单其他信息更新 this::orderExtInfoUpdate */
        self.add_rule::<OrderExtMessage>(
            MessageEventConstants::ORDER_EXT_INFO_UPDATE,
            true,
            noop_consumer(),
        );
        /* 订单状态更新 this::orderStatusUpdate */
        self.add_rule::<OrderStatusMessage>(
            MessageEventConstants::ORDER_STATUS_UPDATE,
            true,
            noop_consumer(),
        );
        /* 售后单更新通知 this::afterSaleStatusUpdate */
        self.add_rule::<AfterSaleMessage>(
            MessageEventConstants::AFTER_SALE_UPDATE,
            true,
            noop_consumer(),
        );
        /* 纠纷更新通知 this::complaintNotify */
        self.add_rule::<ComplaintMessage>(
            MessageEventConstants::COMPLAINT_NOTIFY,
            true,
            noop_consumer(),
        );
        /* 优惠券领取通知 this::couponReceive */
        self.add_rule::<CouponReceiveMessage>(
            MessageEventConstants::RECEIVE_COUPON,
            true,
            noop_consumer(),
        );
        /* 优惠券使用通知 this::couponCreate */
        self.add_rule::<CouponActionMessage>(
            MessageEventConstants::CREATE_COUPON,
            true,
            noop_consumer(),
        );
        /* 优惠券删除通知 this::couponDelete */
        self.add_rule::<CouponActionMessage>(
            MessageEventConstants::DELETE_COUPON,
            true,
            noop_consumer(),
        );
        /* 优惠券过期通知 this::couponExpire */
        self.add_rule::<CouponActionMessage>(
            MessageEventConstants::EXPIRE_COUPON,
            true,
            noop_consumer(),
        );
        /* 更新优惠券信息通知 this::couponUpdate */
        self.add_rule::<CouponActionMessage>(
            MessageEventConstants::UPDATE_COUPON_INFO,
            true,
            noop_consumer(),
        );
        /* 优惠券作废通知 this::couponInvalid */
        self.add_rule::<CouponActionMessage>(
            MessageEventConstants::INVALID_COUPON,
            true,
            noop_consumer(),
        );
        /* 用户优惠券过期通知 this::userCouponExpire */
        self.add_rule::<UserCouponExpireMessage>(
            MessageEventConstants::USER_COUPON_EXPIRE,
            true,
            noop_consumer(),
        );
        /* 优惠券返还通知 this::userCouponUnuse */
        self.add_rule::<UserCouponExpireMessage>(
            MessageEventConstants::USER_COUPON_UNUSE,
            true,
            noop_consumer(),
        );
        /* 优惠券核销通知 this::userCouponUse */
        self.add_rule::<UserCouponExpireMessage>(
            MessageEventConstants::USER_COUPON_USE,
            true,
            noop_consumer(),
        );
        /* 发放团购优惠成功通知 this::voucherSendSucc */
        self.add_rule::<VoucherMessage>(
            MessageEventConstants::VOUCHER_SEND_SUCC,
            true,
            noop_consumer(),
        );
        /* 结算账户变更回调 this::accountNotify */
        self.add_rule::<AccountNotifyMessage>(
            MessageEventConstants::ACCOUNT_NOTIFY,
            true,
            noop_consumer(),
        );
        /* 提现回调 this::withdrawNotify */
        self.add_rule::<WithdrawNotifyMessage>(
            MessageEventConstants::WITHDRAW_NOTIFY,
            true,
            noop_consumer(),
        );
        /* 提现二维码回调 this::qrNotify */
        self.add_rule::<QrNotifyMessage>(
            MessageEventConstants::QRCODE_STATUS,
            true,
            noop_consumer(),
        );
        /* 团长 this::supplierItemUpdate */
        self.add_rule::<SupplierItemMessage>(
            MessageEventConstants::SUPPLIER_ITEM_UPDATE,
            true,
            noop_consumer(),
        );

        /* 用户加入会员 this::vipJoin */
        self.add_rule::<UserInfoMessage>(
            MessageEventConstants::USER_VIP_JOIN,
            false,
            noop_consumer(),
        );
        /* 用户注销会员 this::vipClose */
        self.add_rule::<UserInfoMessage>(
            MessageEventConstants::USER_VIP_CLOSE,
            false,
            noop_consumer(),
        );
        /* 用户等级信息更新 this::vipGradeUpdate */
        self.add_rule::<UserInfoMessage>(
            MessageEventConstants::USER_VIP_GRADE_INFO_UPDATE,
            false,
            noop_consumer(),
        );
        /* 用户积分更新 this::vipScoreUpdate */
        self.add_rule::<UserInfoMessage>(
            MessageEventConstants::USER_VIP_SCORE_UPDATE,
            false,
            noop_consumer(),
        );
        /* 用户积分兑换 this::vipScoreExchange */
        self.add_rule::<ExchangeInfoMessage>(
            MessageEventConstants::USER_VIP_SCORE_EXCHANGE,
            false,
            noop_consumer(),
        );

        /* 分享员变更 this::sharerChange */
        self.add_rule::<SharerChangeMessage>(
            MessageEventConstants::SHARER_CHANGE,
            false,
            noop_consumer(),
        );

        /* 小店注销 this::closeStore */
        self.add_rule::<CloseStoreMessage>(
            MessageEventConstants::CLOSE_STORE,
            true,
            noop_consumer(),
        );
        /* 小店修改名称 this::updateNickname */
        self.add_rule::<NicknameUpdateMessage>(
            MessageEventConstants::SET_SHOP_NICKNAME,
            true,
            noop_consumer(),
        );
    }
}

impl Default for WxChannelMessageServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WxChannelMessageService for WxChannelMessageServiceImpl {
    async fn route(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        service: Option<Arc<dyn WxChannelService>>,
    ) -> Option<String> {
        self.router.route(message, content, app_id, service).await
    }

    fn add_rule(&mut self, rule: Arc<dyn WxChannelMessageRouterRuleErased>) {
        self.router.add_rule(rule);
    }
}

/// 空消费者（对应 Java 基类事件方法的 log-only 行为：消费者仅触发事件方法，
/// 无业务副作用；本仓库无日志依赖，见 [`WxChannelMessageService`] 文档）。
fn noop_consumer<T>() -> HandlerConsumer<T> {
    Arc::new(|_message, _content, _app_id, _context, _session_manager| {})
}
