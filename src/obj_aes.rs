use base64;
use std::str;
use aes::{Aes128, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, Rng};
use hex::encode; // Removed decode to resolve the warning

type Aes256Cbc = Cbc<Aes128, Pkcs7>;

fn get_tenant_key() -> [u8] {
    [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66 ]
}

fn get_iv() -> [u8] {
    [ 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41 ]
}

fn decrypt_aes256_cbc(cipher_text: Vec<u8>) -> String {
    let cipher = Aes256Cbc::new_from_slices(&get_tenant_key(), &get_iv).unwrap();
    let mut decrypted_data = "";

    // Decrypt the data
    let decrypted_temp = cipher.decrypt_vec(&cipher_text);

    match decrypted_temp {
        Ok(data) => {
            decrypted_data = String::from_utf8_lossy(&data);
            println!("Decrypted data: {}", String::from_utf8_lossy(&data));            
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }        
    }
    OK(decrypted_temp)
}