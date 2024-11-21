#![allow(dead_code, unused_variables)]
// rustc utils.rs && ./utils
use hex::decode;
use hmac::{Hmac, Mac};
use num_bigint::BigUint; // todo
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use std::str;
use std::error::Error;
use bitcoin::hashes::{Hash, sha256, ripemd160};
use base58::ToBase58;

type HmacSha512 = Hmac<sha2::Sha512>;

// Constants
const BIP32_HARDENED_OFFSET: u32 = 0x80000000; // for non-hardened child keys, they use pubkey as payload to generate child key which sacrifice safety for flexibility in pubkey's derivition
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"; // order of secp256k1 curve

fn as_hex_bytes(hex_str: &str) -> Vec<u8> {
    decode(hex_str).expect("Invalid hex string")
}

fn hex_to_biguint(hex: &str) -> BigUint {
    let bytes = decode(hex).expect("Invalid hex string");

    BigUint::from_bytes_be(&bytes)
}

fn public_key_from_private_key(private_key: &SecretKey) -> PublicKey {
    let secp = Secp256k1::new();
    PublicKey::from_secret_key(&secp, private_key)
}

fn pubkey_to_address(pubkey: &PublicKey) -> String {
    // Compute SHA-256 of the public key
    let sha256_hash = sha256::Hash::hash(&pubkey.serialize());

    // Compute RIPEMD-160 of the SHA-256 hash
    let ripemd160_hash = ripemd160::Hash::hash(&sha256_hash); // reduce hash size to 20 bytes and add an extra layer security which is also known as pkh

    // Add version byte (0x00 for mainnet addresses)
    let mut versioned_payload = vec![0x00]; // Mainnet version
    versioned_payload.extend_from_slice(&ripemd160_hash);

    // Compute checksum (first 4 bytes of double SHA-256)
    let checksum = sha256::Hash::hash(&sha256::Hash::hash(&versioned_payload));
    let checksum = &checksum[0..4];

    // Add checksum to the end of the payload
    versioned_payload.extend_from_slice(checksum);

    // Base58 encode the versioned payload
    let base58_address = versioned_payload.to_base58();

    base58_address
}

fn derive_child_key(    
    parent_private_key: &str,
    parent_chain_code: &str,
    index: u32,
) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    // Priv_child = Priv_parent + HMAC_SHA512(xpub ∥ index_child)_last_32_bytes
    let mut data = Vec::new();

    if index >= BIP32_HARDENED_OFFSET {
        // Hardened child
        data.push(0x00);
        data.extend_from_slice(&as_hex_bytes(parent_private_key));
    } else {
        // Non-hardened child
        let public_key = public_key_from_private_key(&SecretKey::from_slice(&as_hex_bytes(parent_private_key)).expect("Invalid private key"));
        data.extend_from_slice(&public_key.serialize());
    }  
    data.extend_from_slice(&index.to_be_bytes());

    // Perform HMAC-SHA512
    let mut mac = HmacSha512::new_from_slice(&as_hex_bytes(parent_chain_code)).map_err(|_| "Invalid chain code")?;
    mac.update(&data);
    let result = mac.finalize().into_bytes();

    let (il, ir) = result.split_at(32);
    let il_int = BigUint::from_bytes_be(il);
    let parent_private_key_int = BigUint::from_bytes_be(&as_hex_bytes(parent_private_key));

    let n = hex_to_biguint(N);

    let child_private_key_int = (il_int + parent_private_key_int) % n;
    let child_private_key = child_private_key_int.to_bytes_be();

    Ok((child_private_key, ir.to_vec()))    
}

fn derive_bip_key(
    master_private_key: &str,
    master_chain_code: &str,
    child_index: u32,
    bip_index: Option<u32>,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let bip_index = bip_index.unwrap_or(44); // Bitcoin Improvement Proposals index

    let path = vec![
        bip_index + BIP32_HARDENED_OFFSET, // purpose
        0 + BIP32_HARDENED_OFFSET,         // coin_type
        0 + BIP32_HARDENED_OFFSET,         // account
        0,                                 // change
        child_index,                       // address_index
    ];

    let mut private_key = master_private_key.to_string();
    let mut chain_code = master_chain_code.to_string();

    for index in path {
        match derive_child_key(&private_key, &chain_code, index) {
            Ok((private_key_vec, chain_code_vec)) => {
                // Store hex encoding in a temporary variable before borrowing
                let encoded_private_key = hex::encode(&private_key_vec);
                let encoded_chain_code = hex::encode(&chain_code_vec);

                // Update private_key and chain_code with the new encoded values
                private_key = encoded_private_key; // Update private key (hex string)
                chain_code = encoded_chain_code;
            }
            Err(e) => {return Err(e.into());} // Propagate the error
        }
    }

    Ok((as_hex_bytes(&private_key), as_hex_bytes(&chain_code)))
}

fn main() {
    let master_private_key_hex = "b092cdfb4a05e043560c213f5544002a8798c41f7909cef7cfb4506928d6f5e8"; // aka first half of xpriv(Extended Private Key), xpub = chaincode + master_pubkey
    let master_chain_code_hex =  "85d9f6dc89745a63b1d0da84d7794dfcd8591dbd2aefe59f30fac6d7e0155454"; // aka second half of xpriv(Extended Private Key)
    let index = 0;

    let result = derive_bip_key(master_private_key_hex, master_chain_code_hex, index, Some(44)).expect("");
    let pubkey = public_key_from_private_key(&SecretKey::from_slice(&result.0).expect("Invalid private key"));
    println!("pubkey is {:?}", &pubkey);
    let addr = pubkey_to_address(&pubkey);

    println!("address is: {:?}", addr); 
}
