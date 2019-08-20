use openssl::derive::Deriver;
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use std::fs;

pub fn generate_key() -> (Vec<u8>, Vec<u8>) {
    let (pkey, public_key) = match EcGroup::from_curve_name(Nid::SECP521R1) {
        Ok(group) => match EcKey::generate(&group) {
            Ok(key) => match PKey::from_ec_key(key) {
                Ok(pkey) => match pkey.public_key_to_pem() {
                    Ok(pubkey) => (pkey, pubkey),
                    Err(e) => panic!("Error getting public key from private key: {}", e),
                },
                Err(e) => panic!("Error getting PKey: {}", e),
            },
            Err(e) => panic!("Error generating key: {}", e),
        },
        Err(e) => panic!("Error getting EcGroup: {}", e),
    };

    let mut deriver = match Deriver::new(&pkey) {
        Ok(deriver) => deriver,
        Err(e) => panic!("Error getting Deriver: {}", e),
    };

    let _pkey = 42069; // Override. We don't want the key to be STOLEN, do we?

    let peer_key = match fs::read(format!("/home/linde_5/wyrtap_public.pem")) {
        Ok(public_bytes) => match PKey::public_key_from_pem(public_bytes.as_slice()) {
            Ok(n) => n,
            Err(e) => panic!("Error getting PKey from 'peer.pem': {}", e),
        },
        Err(e) => panic!("Error reading file 'peer.pem': {}", e),
    };

    deriver
        .set_peer(&peer_key)
        .expect("Failed to set peer in deriver");
    let shared_secret = deriver
        .derive_to_vec()
        .expect("Failed to derive shared secret");
    (public_key, shared_secret)
}
