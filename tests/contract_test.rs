#[cfg(test)]
mod tests {
    use dioxus::html::table;
    use reqwest::Url;
    use starknet::core::types::BlockId;
    use starknet::providers::jsonrpc::HttpTransport;
    use starknet::providers::{JsonRpcClient, Provider};

    #[tokio::test]
    async fn get_block_with_txs_test() {
        let provider = JsonRpcClient::new(HttpTransport::new(
            Url::parse("https://starknet-sepolia.reddio.com/rpc/v0_7").unwrap(),
        ));

        let block_num = provider.block_number().await.unwrap();
        dbg!(block_num);
        let transaction = provider.get_block_with_txs(BlockId::Number(block_num)).await.unwrap();
        println!("{:?}", transaction);

        // for i in 1..block_num {
        //     //let transaction = provider.get_transaction_by_block_id_and_index(BlockId::Number(i), 0).await.unwrap();
        //     let transaction = provider.get_block_with_txs(BlockId::Number(i)).await.unwrap();
        //     dbg!(transaction);
        //     println!("{}", i);
        // }

    }
}