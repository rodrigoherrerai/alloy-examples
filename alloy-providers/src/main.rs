#[cfg(test)]
mod tests {
    use alloy_node_bindings::Anvil;
    use alloy_providers::provider::{Provider, TempProvider};

    #[tokio::test]
    async fn test_block_n() {
        let anvil = Anvil::new().spawn();
        let provider = Provider::try_from(&anvil.endpoint()).unwrap();

        let blockn = provider.get_block_number().await.unwrap();

        assert_eq!(blockn, 0);
    }

    #[tokio::test]
    async fn test_bal() {
        let anvil = Anvil::new().spawn();
        let provider = Provider::try_from(&anvil.endpoint()).unwrap();

        let address = provider.get_accounts().await.unwrap()[0];

        let bal = provider.get_balance(address, None).await.unwrap();

        assert_eq!(bal.to_string(), "10000000000000000000000");
    }
}

fn main() {}
