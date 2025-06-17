use num_traits::pow;
// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err(("Transaction data too short").to_string());
    }
    let version = raw_tx_hex[0..8].chars().rev().collect::<Vec<char>>();
    let mut ver_int: u32 = 0;
    for (i, val) in version.iter().enumerate() {
        let value = match val.to_digit(10) {
            Some(c) => c,
            None => return Err("Hex decode error".to_string()),
        };
        ver_int += value * pow(16, version.len() - 1 - i) / 16;
    }
    Ok(ver_int)
}
