use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};

pub fn create_jsonrpc_client(network: Network) -> JsonRpcClient<HttpTransport> {
    match network {
        Network::Mainnet => {
            let provider = JsonRpcClient::new(HttpTransport::new(
                Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
            ));
            return provider;
        }
        Network::Testnet => {
            let provider = JsonRpcClient::new(HttpTransport::new(
                Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
            ));
            return provider;
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_jsonrpc_client() {
        let mainnet_client = create_jsonrpc_client(Network::Mainnet);
        let testnet_client = create_jsonrpc_client(Network::Testnet);
    }
}