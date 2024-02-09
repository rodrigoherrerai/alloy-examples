#[cfg(test)]
mod tests {
    use alloy_primitives::utils::{
        format_ether, format_units, keccak256, parse_ether, parse_units,
    };
    use alloy_primitives::{address, fixed_bytes, Address, FixedBytes, U256};

    #[test]
    fn to_keccak() {
        let str = "hola";
        let hash = keccak256(&str);
        assert_eq!(
            hash.to_string(),
            "0x8aca9664752dbae36135fd0956c956fc4a370feeac67485b49bcd4b99608ae41"
        );
    }

    #[test]
    fn parse() {
        let one_eth: U256 = "1000000000000000000".parse().unwrap();
        let one_gwei: U256 = "1000000000".parse().unwrap();

        let to_eth = parse_ether("1").unwrap().to_string();
        assert_eq!(to_eth, one_eth.to_string());

        let to_gwei = parse_units("1", "gwei").unwrap().to_string();
        assert_eq!(to_gwei, one_gwei.to_string());
    }

    #[test]
    fn format() {
        let one_eth: U256 = "1000000000000000000".parse().unwrap();
        let one_gwei: U256 = "1000000000".parse().unwrap();

        let from_eth = format_ether(one_eth);
        assert_eq!(from_eth.parse::<f64>().unwrap() as i32, 1);

        let from_gwei = format_units(one_gwei, "gwei").unwrap();
        assert_eq!(from_gwei.parse::<f64>().unwrap() as i32, 1);
    }

    #[test]
    fn bytes() {
        let to_bytes: FixedBytes<2> = fixed_bytes!("1111");
        assert_eq!(to_bytes.to_string(), "0x1111");
    }

    #[test]
    fn uint() {
        let x: U256 = U256::from(10);
        let y: U256 = "10".parse().unwrap();

        let r = x + y;

        assert_eq!(r, U256::from(20));
    }

    #[test]
    fn address() {
        let addr_str = "0xb172f70266cF082EB38C0B7FDE42fe5c8028429A";
        let addr: Address = Address::parse_checksummed(addr_str, None).unwrap();

        assert_eq!(addr, address!("b172f70266cF082EB38C0B7FDE42fe5c8028429A"));
    }
}

fn main() {}
