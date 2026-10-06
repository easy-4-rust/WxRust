use wx_rust_store::bean::product::stock::StockFlowResponse;

#[test]
fn stock_flow_unpacks_data_and_retains_negative_changes() {
    let response: StockFlowResponse = serde_json::from_str(r#"{"errcode":0,"data":{"next_key":"page2","stock_flow_info_list":[{"amount":-17,"op_type":2}]}}"#).unwrap();
    assert_eq!(response.next_key.clone().unwrap(), "page2");
    assert_eq!(response.stock_flow_info_list.as_ref().unwrap().len(), 1);
    let value = serde_json::to_value(&response.flow_list.clone().unwrap()[0]).unwrap();
    assert_eq!(value["amount"], -17);
}
