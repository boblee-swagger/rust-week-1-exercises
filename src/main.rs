use rust_week_1_exercises::extract_tx_version;
fn main() {
    let tx_hex = "01000";
    let version = extract_tx_version(tx_hex).unwrap();
    println!("{}", version);
}
