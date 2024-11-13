use hmac::{Hmac, Mac};
use sha2::Sha512;
use pbkdf2::pbkdf2;
use std::string::String;
use hex::encode;

// Constants
const PBKDF2_ROUNDS: u32 = 2048; // Define the number of PBKDF2 rounds here if it's fixed.
const MASTER_KEY: &[u8] = b"Bitcoin seed"; // This is the BIP32 standard for MASTER_KEY

fn normalize_string(s: &str) -> String {
    // Implement normalization if necessary. For now, we assume input is already normalized.
    s.to_string()
}

/// HMAC-SHA512 function to generate the master key
fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha512>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Generate seed from mnemonics
fn seed_from_mnemo(mnemonic: &str, passphrase: &str) -> Vec<u8> {
    let mnemonic = normalize_string(mnemonic);
    let passphrase = normalize_string(passphrase);
    let salt = format!("mnemonic{}", passphrase);
    
    let mut stretched = vec![0u8; 64];
    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), salt.as_bytes(), PBKDF2_ROUNDS, &mut stretched);
    stretched
}

/// Generates the master private key and chain code from the seed
fn masterkey_from_seed(seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let i = hmac_sha512(MASTER_KEY, seed);
    let master_private_key = i[..32].to_vec();
    let master_chain_code = i[32..].to_vec();
    (master_private_key, master_chain_code)
}

fn main() {
    let mnemonic = "fun mix number grab motion region tennis anchor guide pledge tilt job";
    let passphrase = "";
    let seed = seed_from_mnemo(mnemonic, passphrase);
    println!("seed is: {:x?}", encode(seed.clone())); // it can be checked at: https://learnmeabitcoin.com/technical/keys/hd-wallets/mnemonic-seed/
    
    let (master_private_key, master_chain_code) = masterkey_from_seed(&seed);

    println!("Master Private Key: {:x?}", encode(master_private_key.clone())); // it can be checked at: https://learnmeabitcoin.com/technical/keys/hd-wallets/extended-keys/
    println!("Master Chain Code: {:x?}", encode(master_chain_code.clone()));
}
