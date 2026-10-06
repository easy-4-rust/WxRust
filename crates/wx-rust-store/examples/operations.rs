use wx_rust_common::error::WxErrorException;
use wx_rust_store::api::WxStoreService;

/// 展示商品、地址、订单、售后与资金查询入口；由业务方传入已有客户端与有效单号。
/// 返回任一步查询的错误，调用方应按业务错误码决定处理方式。
pub async fn inspect_operations(
    service: &dyn WxStoreService,
    product_id: String,
    order_id: String,
    after_sale_id: String,
) -> Result<(), WxErrorException> {
    let missing = || WxErrorException::from_code(-99, "经营子服务未装配");
    let product = service.product_service().ok_or_else(missing)?;
    let _product = product.get_product(product_id, Some(1)).await?;
    let address = service.address_service().ok_or_else(missing)?;
    let _addresses = address.list_address(Some(0), Some(20)).await?;
    let order = service.order_service().ok_or_else(missing)?;
    let _order = order.get_order(order_id).await?;
    let _carriers = order.list_delivery_company().await?;
    let after_sale = service.after_sale_service().ok_or_else(missing)?;
    let _after_sale = after_sale.get_after_sale(after_sale_id).await?;
    let fund = service.fund_service().ok_or_else(missing)?;
    let _balance = fund.get_balance().await?;
    Ok(())
}

fn main() {
    // 示例只编译查询流程；接入后在业务异步上下文调用，不在此处访问微信。
}
