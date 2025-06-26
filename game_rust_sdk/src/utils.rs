use anyhow::Result;
use ethers::types::{Address, U256};
use std::str::FromStr;

/// Parse a string slice into an Ethereum address
pub fn parse_address(address: &str) -> Result<Address> {
    Ok(Address::from_str(address)?)
}

/// Parse a string slice into a U256 (big number)
pub fn parse_u256(value: &str) -> Result<U256> {
    if value.starts_with("0x") {
        return Ok(U256::from_str_radix(&value[2..], 16)?);
    }
    Ok(U256::from_dec_str(value)?)
}

/// Convert a decimal number with specified decimals to wei equivalent
pub fn to_wei(amount: f64, decimals: u8) -> Result<U256> {
    let mut wei_amount = amount;
    for _ in 0..decimals {
        wei_amount *= 10.0;
    }
    Ok(U256::from(wei_amount as u128))
}

/// Format a U256 value to a human-readable string with specified decimals
pub fn format_units(value: U256, decimals: u8) -> String {
    let mut value_str = value.to_string();
    
    // Pad with leading zeros if necessary
    if value_str.len() <= decimals as usize {
        value_str = "0".repeat(decimals as usize + 1 - value_str.len()) + &value_str;
    }
    
    // Insert decimal point
    let decimal_pos = value_str.len() - decimals as usize;
    if decimal_pos < value_str.len() {
        value_str.insert(decimal_pos, '.');
        
        // Trim trailing zeros
        while value_str.ends_with('0') && value_str.contains('.') {
            value_str.pop();
        }
        
        // Remove decimal point if it's the last character
        if value_str.ends_with('.') {
            value_str.pop();
        }
    }
    
    value_str
} 