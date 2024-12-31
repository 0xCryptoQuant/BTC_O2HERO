use hex::{decode, encode};
use bitcoin::hashes::{Hash, sha256};
use std::collections::HashMap;

enum RawTxValue {
    Single(String),
    List(Vec<String>),
}

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    decode(hex_str).expect("Invalid hex string") // let it panic if hex str is invalid
}

fn bytes_to_hex(raw_str: &[u8]) -> String {
    encode(raw_str)
}

fn main() {
    // Preimage for P2SH
    let preimage = b"Have fun staying poor!";
    let preimage_hex = bytes_to_hex(preimage);

    // SHA256 digest of the preimage
    let lock = sha256::Hash::hash(preimage);
    let lock_hex = bytes_to_hex(&lock);

    // Bitcoin script: OP_SHA256 <lock_hex> OP_EQUAL
    let redeem_script_hex = format!("a820{}87", lock_hex);

    let funding_txid_be = "3ed61ee6595475092bc5b850c9d7859d7cc4825917065987c1cebe12b17ed361"; // funding btc to p2sh
    let funding_txid_le: Vec<u8> = hex_to_bytes(funding_txid_be).into_iter().rev().collect();
    let funding_txid_le_hex = bytes_to_hex(&funding_txid_le);

    // Input skeleton for a SegWit tx
    let vout = "00000000";
    let sequence = "ffffffff";
    let script_sig = "00";

    let input_hex = format!(
        "{}{}{}{}",
        funding_txid_le_hex, vout, script_sig, sequence
    );

    let input_witness_stack_size = "02"; // size in hex: [witness_data1, witness_data2, ...]
    let preimage_len = format!("{:02x}", preimage.len());
    let redeem_script_len = format!("{:02x}", redeem_script_hex.len() / 2);

    let witness_hex = format!(
        "{}{}{}{}{}",
        input_witness_stack_size, preimage_len, preimage_hex, redeem_script_len, redeem_script_hex
    );
    
    let raw_tx_hex_no_input = "80f0fa02000000001600146a4f06e4fd11e1491b219dcc08deb23e41049a63"; // btc_amount + scriptPubKey

    let mut raw_tx: HashMap<&str, RawTxValue> = HashMap::new();
    raw_tx.insert("marker_and_flag", RawTxValue::Single("0001".to_string()));
    raw_tx.insert("version", RawTxValue::Single("01000000".to_string()));
    raw_tx.insert("input_count", RawTxValue::Single("01".to_string()));
    raw_tx.insert("inputs", RawTxValue::List(vec![input_hex]));
    raw_tx.insert("output_count", RawTxValue::Single("01".to_string()));
    raw_tx.insert(
        "outputs",
        RawTxValue::List(vec![raw_tx_hex_no_input.to_string()]),
    );
    raw_tx.insert("locktime", RawTxValue::Single("00000000".to_string()));
    raw_tx.insert("witness", RawTxValue::Single(witness_hex));

    let mut raw_tx_final_hex = String::new();

    if let (
        Some(RawTxValue::Single(version)),
        Some(RawTxValue::Single(marker_and_flag)),
        Some(RawTxValue::Single(input_count)),
        Some(RawTxValue::List(inputs)),
        Some(RawTxValue::Single(output_count)),
        Some(RawTxValue::List(outputs)),
        Some(RawTxValue::Single(witness)),
        Some(RawTxValue::Single(locktime)),
    ) = (
        raw_tx.get("version"),
        raw_tx.get("marker_and_flag"),
        raw_tx.get("input_count"),
        raw_tx.get("inputs"),
        raw_tx.get("output_count"),
        raw_tx.get("outputs"),
        raw_tx.get("witness"),
        raw_tx.get("locktime"),
    ) {
        raw_tx_final_hex = format!(
            "{}{}{}{}{}{}{}{}",
            version, marker_and_flag, input_count, inputs[0], output_count, outputs[0], witness, locktime
        );
    }

    println!("raw_tx_final_hex: {}", raw_tx_final_hex);
}
