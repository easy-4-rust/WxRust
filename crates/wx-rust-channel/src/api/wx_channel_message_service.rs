//! 消息服务接口。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.BaseWxChannelMessageService`
//! （Wave 2 H2c 迁移）：路由入口 + 规则注册 + 40 个回调事件处理方法
//! （订单/商品/售后/优惠券/资金/会员/分享员/店铺/团长/团购券）。
//!
//! Java 为纯入站回调服务（无消息发送/回执/客服等出站 API——任务说明中的
//! “消息发送”在本子系统中不存在，见 Java 接口全量方法核对）。40 个事件
//! 方法在 Java 抽象基类 `BaseWxChannelMessageServiceImpl` 中仅有 log 行为
//! （`log.info("xxx:{}", JsonUtils.encode(message))`）；本仓库无日志依赖，
//! Rust 以 trait 默认空实现表达同一“无业务副作用”语义（ADAPTED），
//! 业务扩展通过自定义路由规则（handler）完成。

use std::sync::Arc;

use async_trait::async_trait;
use wx_rust_common::session::WxSessionManager;

use crate::api::WxChannelService;
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
use crate::bean::message::store::{CloseStoreMessage, NicknameUpdateMessage};
use crate::bean::message::supplier::SupplierItemMessage;
use crate::bean::message::vip::{ExchangeInfoMessage, UserInfoMessage};
use crate::bean::message::voucher::VoucherMessage;
use crate::message::{RouteContext, WxChannelMessage, WxChannelMessageRouterRuleErased};

/// 消息服务（对应 Java `BaseWxChannelMessageService`）。
#[async_trait]
pub trait WxChannelMessageService: Send + Sync {
    /// 路由微信消息（对应 Java `route`）。
    ///
    /// # 参数
    /// - `message`：消息（已按基础字段解析）
    /// - `content`：消息原始内容（规则按 `messageClass` 重新反序列化用）
    /// - `app_id`：appId
    /// - `service`：服务实例（提供 `msgDataFormat` 配置；可为空）
    ///
    /// # 返回
    /// 最后一个同步规则 handler 的结果（无匹配规则或全部异步时为 `None`）
    async fn route(
        &self,
        message: &WxChannelMessage,
        content: &str,
        app_id: &str,
        service: Option<Arc<dyn WxChannelService>>,
    ) -> Option<String>;

    /// 添加一条规则进入路由器（对应 Java `addRule`；Rust 需要 `&mut self`
    /// 以修改规则表，ADAPTED：Java 无可变性概念）。
    fn add_rule(&mut self, rule: Arc<dyn WxChannelMessageRouterRuleErased>);

    /// 订单下单（对应 Java `orderNew`）。
    #[allow(clippy::unused_self)]
    fn order_new(
        &self,
        _message: &OrderIdMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单下单:{}", JsonUtils.encode(message))
    }

    /// 订单取消（对应 Java `orderCancel`）。
    #[allow(clippy::unused_self)]
    fn order_cancel(
        &self,
        _message: &OrderCancelMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单取消:{}", JsonUtils.encode(message))
    }

    /// 订单支付成功（对应 Java `orderPay`）。
    #[allow(clippy::unused_self)]
    fn order_pay(
        &self,
        _message: &OrderPayMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单支付成功:{}", JsonUtils.encode(message))
    }

    /// 订单待发货（对应 Java `orderWaitShipping`）。
    #[allow(clippy::unused_self)]
    fn order_wait_shipping(
        &self,
        _message: &OrderIdMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单待发货:{}", JsonUtils.encode(message))
    }

    /// 订单发货（对应 Java `orderDelivery`）。
    #[allow(clippy::unused_self)]
    fn order_delivery(
        &self,
        _message: &OrderDeliveryMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单发货:{}", JsonUtils.encode(message))
    }

    /// 订单确认收货（对应 Java `orderConfirm`）。
    #[allow(clippy::unused_self)]
    fn order_confirm(
        &self,
        _message: &OrderConfirmMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单确认收货:{}", JsonUtils.encode(message))
    }

    /// 订单结算成功（对应 Java `orderSettle`）。
    #[allow(clippy::unused_self)]
    fn order_settle(
        &self,
        _message: &OrderSettleMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单结算:{}", JsonUtils.encode(message))
    }

    /// 订单其他信息更新（对应 Java `orderExtInfoUpdate`）。
    #[allow(clippy::unused_self)]
    fn order_ext_info_update(
        &self,
        _message: &OrderExtMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单其他信息更新:{}", JsonUtils.encode(message))
    }

    /// 订单状态更新（对应 Java `orderStatusUpdate`）。
    #[allow(clippy::unused_self)]
    fn order_status_update(
        &self,
        _message: &OrderStatusMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("订单状态更新:{}", JsonUtils.encode(message))
    }

    /// 商品审核结果（对应 Java `spuAudit`）。
    #[allow(clippy::unused_self)]
    fn spu_audit(
        &self,
        _message: &SpuAuditMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("商品审核:{}", JsonUtils.encode(message))
    }

    /// 商品系统下架通知（对应 Java `spuStatusUpdate`）。
    #[allow(clippy::unused_self)]
    fn spu_status_update(
        &self,
        _message: &SpuAuditMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("商品状态更新:{}", JsonUtils.encode(message))
    }

    /// 商品更新通知（对应 Java `spuUpdate`）。
    #[allow(clippy::unused_self)]
    fn spu_update(
        &self,
        _message: &SpuAuditMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("商品更新:{}", JsonUtils.encode(message))
    }

    /// 商品库存不足通知（对应 Java `stockNoEnough`）。
    #[allow(clippy::unused_self)]
    fn stock_no_enough(
        &self,
        _message: &SpuStockMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("商品库存不足:{}", JsonUtils.encode(message))
    }

    /// 类目审核结果（对应 Java `categoryAudit`）。
    #[allow(clippy::unused_self)]
    fn category_audit(
        &self,
        _message: &CategoryAuditMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("分类审核:{}", JsonUtils.encode(message))
    }

    /// 品牌更新（对应 Java `brandUpdate`）。
    #[allow(clippy::unused_self)]
    fn brand_update(
        &self,
        _message: &BrandMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("品牌更新:{}", JsonUtils.encode(message))
    }

    /// 售后单状态更新（对应 Java `afterSaleStatusUpdate`）。
    #[allow(clippy::unused_self)]
    fn after_sale_status_update(
        &self,
        _message: &AfterSaleMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("售后状态更新:{}", JsonUtils.encode(message))
    }

    /// 纠纷回调（对应 Java `complaintNotify`）。
    #[allow(clippy::unused_self)]
    fn complaint_notify(
        &self,
        _message: &ComplaintMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("投诉通知:{}", JsonUtils.encode(message))
    }

    /// 用户领券通知（对应 Java `couponReceive`）。
    #[allow(clippy::unused_self)]
    fn coupon_receive(
        &self,
        _message: &CouponReceiveMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券领取:{}", JsonUtils.encode(message))
    }

    /// 创建优惠券通知（对应 Java `couponCreate`）。
    #[allow(clippy::unused_self)]
    fn coupon_create(
        &self,
        _message: &CouponActionMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券创建:{}", JsonUtils.encode(message))
    }

    /// 优惠券删除通知（对应 Java `couponDelete`）。
    #[allow(clippy::unused_self)]
    fn coupon_delete(
        &self,
        _message: &CouponActionMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券删除:{}", JsonUtils.encode(message))
    }

    /// 优惠券过期通知（对应 Java `couponExpire`）。
    #[allow(clippy::unused_self)]
    fn coupon_expire(
        &self,
        _message: &CouponActionMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券过期:{}", JsonUtils.encode(message))
    }

    /// 更新优惠券信息通知（对应 Java `couponUpdate`）。
    #[allow(clippy::unused_self)]
    fn coupon_update(
        &self,
        _message: &CouponActionMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券更新:{}", JsonUtils.encode(message))
    }

    /// 优惠券作废通知（对应 Java `couponInvalid`）。
    #[allow(clippy::unused_self)]
    fn coupon_invalid(
        &self,
        _message: &CouponActionMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("优惠券失效:{}", JsonUtils.encode(message))
    }

    /// 用户优惠券过期通知（对应 Java `userCouponExpire`）。
    #[allow(clippy::unused_self)]
    fn user_coupon_expire(
        &self,
        _message: &UserCouponExpireMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户优惠券过期:{}", JsonUtils.encode(message))
    }

    /// 用户优惠券使用通知（对应 Java `userCouponUse`；Java 参数类型即
    /// `UserCouponExpireMessage`，照抄）。
    #[allow(clippy::unused_self)]
    fn user_coupon_use(
        &self,
        _message: &UserCouponExpireMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户优惠券使用:{}", JsonUtils.encode(message))
    }

    /// 用户优惠券返还通知（对应 Java `userCouponUnuse`）。
    #[allow(clippy::unused_self)]
    fn user_coupon_unuse(
        &self,
        _message: &UserCouponExpireMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户优惠券取消使用:{}", JsonUtils.encode(message))
    }

    /// 发放团购优惠成功回调（对应 Java `voucherSendSucc`）。
    #[allow(clippy::unused_self)]
    fn voucher_send_succ(
        &self,
        _message: &VoucherMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("发放团购优惠成功:{}", JsonUtils.encode(message))
    }

    /// 结算账户变更回调（对应 Java `accountNotify`）。
    #[allow(clippy::unused_self)]
    fn account_notify(
        &self,
        _message: &AccountNotifyMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("账户通知:{}", JsonUtils.encode(message))
    }

    /// 提现回调（对应 Java `withdrawNotify`）。
    #[allow(clippy::unused_self)]
    fn withdraw_notify(
        &self,
        _message: &WithdrawNotifyMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("提现通知:{}", JsonUtils.encode(message))
    }

    /// 提现二维码回调（对应 Java `qrNotify`）。
    #[allow(clippy::unused_self)]
    fn qr_notify(
        &self,
        _message: &QrNotifyMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("二维码通知:{}", JsonUtils.encode(message))
    }

    /// 团长商品变更（对应 Java `supplierItemUpdate`）。
    #[allow(clippy::unused_self)]
    fn supplier_item_update(
        &self,
        _message: &SupplierItemMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("供应商商品更新:{}", JsonUtils.encode(message))
    }

    /// 用户加入会员（对应 Java `vipJoin`）。
    #[allow(clippy::unused_self)]
    fn vip_join(
        &self,
        _message: &UserInfoMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户加入会员:{}", JsonUtils.encode(message))
    }

    /// 用户注销会员（对应 Java `vipClose`）。
    #[allow(clippy::unused_self)]
    fn vip_close(
        &self,
        _message: &UserInfoMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户注销会员:{}", JsonUtils.encode(message))
    }

    /// 用户等级更新（对应 Java `vipGradeUpdate`）。
    #[allow(clippy::unused_self)]
    fn vip_grade_update(
        &self,
        _message: &UserInfoMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户等级信息更新:{}", JsonUtils.encode(message))
    }

    /// 用户积分更新（对应 Java `vipScoreUpdate`）。
    #[allow(clippy::unused_self)]
    fn vip_score_update(
        &self,
        _message: &UserInfoMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户积分更新:{}", JsonUtils.encode(message))
    }

    /// 用户积分兑换（对应 Java `vipScoreExchange`）。
    #[allow(clippy::unused_self)]
    fn vip_score_exchange(
        &self,
        _message: &ExchangeInfoMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("用户积分兑换:{}", JsonUtils.encode(message))
    }

    /// 小店注销（对应 Java `closeStore`）。
    #[allow(clippy::unused_self)]
    fn close_store(
        &self,
        _message: &CloseStoreMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("小店注销:{}", JsonUtils.encode(message))
    }

    /// 小店修改名称（对应 Java `updateNickname`）。
    #[allow(clippy::unused_self)]
    fn update_nickname(
        &self,
        _message: &NicknameUpdateMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("小店修改名称:{}", JsonUtils.encode(message))
    }

    /// 默认消息处理（对应 Java `defaultMessageHandler`，返回 null）。
    #[allow(clippy::unused_self)]
    fn default_message_handler(
        &self,
        _message: &WxChannelMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) -> Option<String> {
        // Java: log.info("默认消息处理:{}", JsonUtils.encode(message)); return null;
        None
    }

    /// 分享员变更（对应 Java `sharerChange`）。
    #[allow(clippy::unused_self)]
    fn sharer_change(
        &self,
        _message: &WxChannelMessage,
        _content: &str,
        _app_id: &str,
        _context: &mut RouteContext,
        _session_manager: &dyn WxSessionManager,
    ) {
        // Java: log.info("分享员变更:{}", JsonUtils.encode(message))
    }
}
