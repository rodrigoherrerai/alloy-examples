#[cfg(test)]
mod test {
    use alloy_signer::{LocalWallet, Signer, SignerSync};

    #[test]
    fn test_random_wallet() {
        let wallet = LocalWallet::random();
        let addr = wallet.address();
        assert_eq!(addr.len(), 20);
    }

    #[test]
    fn test_wallet_from_pk() {
        let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
            .parse::<LocalWallet>()
            .unwrap();

        assert_eq!(wallet.address().len(), 20);
    }

    #[test]
    fn test_sig() {
        let wallet = LocalWallet::random();
        let message = "hey";
        let sig = wallet.sign_message_sync(message.as_bytes()).unwrap();

        let recovered = sig.recover_address_from_msg(message).unwrap();
        assert_eq!(recovered, wallet.address());
    }
}

fn main() {}
