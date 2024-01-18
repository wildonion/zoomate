

use wallexerr::misc::*;
use crate::constants;


/** 
     ---------------------------------------------------------------------
    |          EdDSA Ed25519 WITH SYMMETRIC SIGNING SUING AES256
    |---------------------------------------------------------------------
    |
    | CURVE           -> ed25519
    | DATA ENCRYPTION -> SYMMETRIC WIGH AES256 ALGORITHM
    | RETURN DATA     -> base58 Signature
    |

**/
pub mod eddsa_with_symmetric_signing{

    

    pub use super::*;
    
    pub fn ed25519_aes256_signing(data: &str, mut wallet: Wallet) -> String{

        // note that nonce must be unique per each user or a unique identity
        let mut default_aes256_config = &mut Aes256Config::default();
        default_aes256_config.secret_key = constants::gen_random_chars(64); /*** ---- secret key must be 64 bytes or 512 bits */
        default_aes256_config.nonce = constants::gen_random_chars(16); /*** ---- secret key must be 16 bytes or 128 bits */
        default_aes256_config.data = data.as_bytes().to_vec();

        let edprvkey = wallet.ed25519_secret_key.clone().unwrap();
        let base58_sig = wallet.self_ed25519_aes256_sign(
            &edprvkey, 
            default_aes256_config
        );
        
        /* default_aes256_config.data now contains the aes256 hash of the raw data */
        let hash_of_data = default_aes256_config.clone().data;
        println!("aes256 encrypted data :::: {:?}", hex::encode(&hash_of_data));
        println!("signature :::: {:?}", base58_sig.clone());
        
        let is_verified = wallet.self_verify_ed25519_signature(
            &base58_sig.clone().unwrap(), 
            &hash_of_data, 
            &wallet.clone().ed25519_public_key.unwrap()
        );
        
        match is_verified{
            Ok(is_verified) => {

                default_aes256_config.data = hash_of_data.clone(); /* update data field with encrypted form of raw data */
                let dec = wallet.self_generate_data_from_aes256(default_aes256_config);
                println!("aes256 decrypted data :::: {:?}", std::str::from_utf8(&dec));

                let deserialized_data = std::str::from_utf8(&dec).unwrap();
                if deserialized_data == data{

                    println!("✅ got same data");
                    return base58_sig.unwrap();

                } else{

                    eprintln!("🔴 invalid data");
                    return String::from("");
                }

            },
            Err(e) => return String::from("")
        }

    }


    pub fn ed25519_secure_cell_signing(data: &str, mut wallet: Wallet) -> String{

        let mut default_secure_cell_config = &mut SecureCellConfig::default();
        // following secret key is the sha3 keccak256 hash of random chars
        default_secure_cell_config.secret_key = {
            hex::encode(
                wallet.self_generate_keccak256_hash_from(
                    &constants::gen_random_chars(64)
                )
            )
        };
        default_secure_cell_config.data = data.as_bytes().to_vec();

        let edprvkey = wallet.ed25519_secret_key.clone().unwrap();
        let base58_sig = wallet.self_ed25519_secure_cell_sign(
            &edprvkey, 
            default_secure_cell_config
        );

        /* default_secure_cell_config.data now contains the aes256 hash of the raw data */
        let hash_of_data = default_secure_cell_config.clone().data;
        println!("secure cell aes256 encrypted data :::: {:?}", hex::encode(&hash_of_data));
        println!("signature :::: {:?}", base58_sig.clone());
        
        let is_verified = wallet.self_verify_ed25519_signature(
            &base58_sig.clone().unwrap(), 
            &hash_of_data, 
            &wallet.clone().ed25519_public_key.unwrap()
        );
        
        match is_verified{
            Ok(is_verified) => {

                default_secure_cell_config.data = hash_of_data.clone(); /* update data field with encrypted form of raw data */
                let dec = wallet.self_secure_cell_decrypt(default_secure_cell_config).unwrap();
                println!("aes256 decrypted data :::: {:?}", std::str::from_utf8(&dec));

                let deserialized_data = std::str::from_utf8(&dec).unwrap();
                if deserialized_data == data{

                    println!("✅ got same data");
                    return base58_sig.unwrap();

                } else{

                    eprintln!("🔴 invalid data");
                    return String::from("");
                }

            },
            Err(e) => return String::from("")
        }

    }

    
}

/** 
     ---------------------------------------------------------------------
    |          EdDSA Ed25519 USING KECCAK256
    |---------------------------------------------------------------------
    |
    | CURVE           -> ed25519
    | DATA ENCRYPTION -> KECCAK256
    | RETURN DATA     -> base58 Signature
    |

**/
pub mod eddsa_with_keccak256_signing{

    pub use super::*;
    
    pub fn ed25519_keccak256_signing(data: &str, mut wallet: Wallet) -> String{

        let edprvkey = wallet.ed25519_secret_key.clone().unwrap();
        let base58_sig = wallet.self_ed25519_sign(
            data,
            &edprvkey, 
        );

        let hash_of_data = wallet.self_generate_keccak256_hash_from(data);
        let is_verified = wallet.self_verify_ed25519_signature(
            &base58_sig.clone().unwrap(), 
            &hash_of_data, 
            &wallet.clone().ed25519_public_key.unwrap()
        );
        
        match is_verified{
            Ok(is_verified) => {
                
                return base58_sig.unwrap();

            },
            Err(e) => return String::from("")
        }

    }
    
}