use hex::{decode, encode};
use bitcoin::hashes::{Hash, sha256};
use std::collections::HashMap;

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    decode(hex_str).expect("Invalid hex string") // let it panic if hex str is invalid
}

fn bytes_to_hex(raw_str: &[u8]) -> String {
    encode(raw_str)
}

fn main() {
    // preimage for p2sh
    let preimage = b"Have fun staying poor!"; // &[u8] by default
    let preimage_hex = bytes_to_hex(preimage);
    
    // sha256 digest of the preimage
    let lock = sha256::Hash::hash(preimage); // &[u8] for input
    let lock_hex = bytes_to_hex(&lock);

    // bitcoin script: OP_SHA256 <lock_hex> OP_EQUAL 
    let redeem_script_hex = format!("{}{}{}{}", "a8", "20", lock_hex, "87");
    println!("redeem script is: {:?}", redeem_script_hex);

    // tx for locking BTC to the "p2sh" address:
    let funding_txid_be = "dd986640889214c882033c1bf3f6b91d26415b3bbdc826dc854bd9bf9e45f8af"; // big endian
    let funding_txid_le: Vec<u8> = hex_to_bytes(funding_txid_be).into_iter().rev().collect(); // little endian
    let funding_txid_le_hex = bytes_to_hex(&funding_txid_le);
    
    println!("funding tx id little endian is: {:?}", funding_txid_le_hex);

    // input skeleton for a legacy tx (including txid
    let vout = "00000000";
    let sequence = "ffffffff";
    let preimage_len_hex = format!("{:02x}", preimage.len() as u8);
    let redeem_script_hex_len_hex = format!("{:02x}", redeem_script_hex.len() / 2);
    // scriptSig is one of tx input field for unlocking linked utxo
    let script_sig = format!(
        "{}{}{}{}",
        preimage_len_hex, preimage_hex, redeem_script_hex_len_hex, redeem_script_hex
    );
    let script_sig_len = (script_sig.len() / 2) as u8;
    let script_sig_len_hex = format!("{:02x}", script_sig_len);

    println!("scriptSig: {}", script_sig);
    println!("scriptSig_len: {}", script_sig_len_hex);

    let mut input = HashMap::new();
    input.insert("txid", funding_txid_le_hex.as_str());
    input.insert("vout", vout);
    input.insert("scriptSig_len", script_sig_len_hex.as_str());
    input.insert("scriptSig", &script_sig);
    input.insert("sequence", sequence);

    let input_hex = format!(
        "{}{}{}{}{}",
        input["txid"], input["vout"], input["scriptSig_len"], input["scriptSig"], input["sequence"]
    );

    // raw tx hex got by using bitocin-cli: bitcoin-cli -regtest createrawtransaction ...
    let raw_tx_hex_no_input = "02000000000100e1f5050000000016001496022cc7da5b884bd87fd0fa36c3ac76cd51206500000000";

    let version = "02000000"; 
    let input_count = "01"; 
    let raw_tx_done = format!(
        "{}{}{}{}",
        version,
        input_count,
        input_hex,
        &raw_tx_hex_no_input[10..] // Skip the first 10 characters of the raw_tx_hex_no_input
    );

    println!("raw tx hex is: {}", raw_tx_done);
    // raw tx hex: 0200000001aff8459ebfd94b85dc26c8bd3b5b41261db9f6f31b3c0382c8149288406698dd000000003b16486176652066756e2073746179696e6720706f6f722123a820cc7bdbddabfde0a72ff5bb0901e3fde7c4ae50311b611387ff51cbf606d2808987ffffffff0100e1f5050000000016001496022cc7da5b884bd87fd0fa36c3ac76cd51206500000000
    // txid: d8f1518399fcc9f82b3938c28011970b1534bf2474816bc647ca37dc3e6f6f4c
}



