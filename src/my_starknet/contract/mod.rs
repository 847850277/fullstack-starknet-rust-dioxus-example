use provider::create_jsonrpc_client;
use starknet::core::types::{BlockId, BlockTag, FieldElement, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::providers::jsonrpc::HttpTransport;

use crate::my_starknet::provider;

pub async fn call_contract_read_function(provider: JsonRpcClient<HttpTransport>, contract_address: FieldElement, selector: String, call_data: Vec<FieldElement>) -> Vec<FieldElement> {
    let call_result =
        provider
            .call(
                FunctionCall {
                    contract_address: contract_address,
                    entry_point_selector: get_selector_from_name(&selector).unwrap(),
                    calldata: call_data,
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .expect("failed to call contract");

    //dbg!(call_result);
    return call_result;
}
