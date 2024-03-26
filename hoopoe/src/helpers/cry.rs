


use wallexerr::misc::*;
use crate::helpers::misc::{gen_random_chars};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;


/*  -------------------------------------------------------------------------
    0 - generate ed25519 wallet and AES256 secure cell config and share between trusted parties
    1 - encrypt data using AES256
    2 - sign AES256 hash of data using pvkey
    3 - send the AES256 hash of signatre to client
    4 - use the secure cell config to decrypt the signature
    5 - use signature, pubkey and AES256 hash of data to verify the signature
    6 - the connection between parties is now secured
*/


pub mod wannacry{

    pub use super::*;

    pub async fn secure_session(){

        // use to secure the communication between client and server
        // ...

    }

    pub async fn encrypt_file(fpath: &str) -> (Vec<u8>, SecureCellConfig){

        let file = tokio::fs::File::open(fpath).await;
    
        let mut buffer = vec![]; // file content in form of utf8
        file.unwrap().read_to_end(&mut buffer).await; // await on it to fill the buffer
    
        let mut wallet = wallexerr::misc::Wallet::new_ed25519();
        let mut default_secure_cell_config = &mut SecureCellConfig::default();
        // secret key is the keccak256 hash of a random 64 bytes chars or 4096 hex chars
        default_secure_cell_config.secret_key = {
            hex::encode(
                wallet.self_generate_keccak256_hash_from(
                    &gen_random_chars(64)
                )
            )
        };
    
        // store the config into a json file so we can use later to decrypt the file
        let config_path = format!("{}.config.json", fpath);
        let config_file = tokio::fs::File::create(config_path).await;
        config_file.unwrap().write_all(
            &serde_json::to_string_pretty(&default_secure_cell_config).unwrap().as_bytes()
        ).await;
        default_secure_cell_config.data = buffer;
    
        let encrypted_data = wallet.self_secure_cell_encrypt(default_secure_cell_config).unwrap();
        default_secure_cell_config.data = encrypted_data.clone(); //*** important part */
    
        let enc_file_path = format!("{}.enc", fpath);
        let file = tokio::fs::File::create(&enc_file_path).await;
        file.unwrap().write_all(&encrypted_data).await;
    
        (encrypted_data, default_secure_cell_config.to_owned())
    
    }
    
    pub async fn decrypt_file(decpath: &str, default_secure_cell_config: &mut SecureCellConfig) -> Vec<u8>{
    
        let file = tokio::fs::File::create(decpath).await;
    
        let mut wallet = wallexerr::misc::Wallet::new_ed25519();
        let decrypted_data = wallet.self_secure_cell_decrypt(default_secure_cell_config).unwrap();
    
        file.unwrap().write_all(&decrypted_data).await;
    
        decrypted_data
    
    }
    
}


/** 
     ---------------------------------------------------------------------
    |          EdDSA Ed25519 WITH SYMMETRIC SIGNING SUING AES256
    |---------------------------------------------------------------------
    |
    | CURVE           -> ed25519
    | DATA ENCRYPTION -> SYMMETRIC WITH AES256 ALGORITHM
    | RETURN DATA     -> base58 Signature
    |

**/
pub mod eddsa_with_symmetric_signing{

    pub use super::*;

    pub fn ed25519_encrypt_and_sign_tcp_packet_with_aes256_secure_cell(mut wallet: Wallet, aes256_config: &mut SecureCellConfig) -> String{

        let raw_data_vec = aes256_config.clone().data;
        let raw_data_str = std::str::from_utf8(&raw_data_vec).unwrap();

        let edprvkey = wallet.ed25519_secret_key.clone().unwrap();
        let base58_sig = wallet.self_ed25519_secure_cell_sign(
            &edprvkey, 
            aes256_config
        );

        /* aes256_config.data now contains the aes256 hash of the raw data */
        let hash_of_data = aes256_config.clone().data;
        println!("secure cell aes256 encrypted data :::: {:?}", hex::encode(&hash_of_data));
        println!("signature :::: {:?}", base58_sig.clone());
        
        let is_verified = wallet.self_verify_ed25519_signature(
            &base58_sig.clone().unwrap(), 
            &hash_of_data, 
            &wallet.clone().ed25519_public_key.unwrap()
        );
        
        match is_verified{
            Ok(is_verified) => {

                aes256_config.data = hash_of_data.clone(); /* update data field with encrypted form of raw data */
                let dec = wallet.self_secure_cell_decrypt(aes256_config).unwrap();
                println!("aes256 decrypted data :::: {:?}", std::str::from_utf8(&dec));

                let deserialized_data = std::str::from_utf8(&dec).unwrap();
                if deserialized_data == raw_data_str{

                    wallet.self_save_to_json("ed25519-secure_cell");
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

    pub fn ed25519_decrypt_and_verify_tcp_packet_with_aes256_secure_cell(mut wallet: Wallet, base58_sig: &str, aes256_config: &mut SecureCellConfig) -> (bool, String){

        /* aes256_config.data now contains the aes256 hash of the raw data */
        let hash_of_data = aes256_config.clone().data;
        println!("secure cell aes256 encrypted data :::: {:?}", hex::encode(&hash_of_data));
        println!("signature :::: {:?}", base58_sig);
        
        let is_verified = wallet.self_verify_ed25519_signature(
            &base58_sig, 
            &hash_of_data, 
            &wallet.clone().ed25519_public_key.unwrap()
        );
        
        match is_verified{
            Ok(is_verified) => {

                aes256_config.data = hash_of_data.clone(); /* update data field with encrypted form of raw data */
                let dec = wallet.self_secure_cell_decrypt(aes256_config).unwrap();
                println!("aes256 decrypted data :::: {:?}", std::str::from_utf8(&dec)); // dec is not the vector of hex it's the raw vector of data so we can map it to str like this

                let deserialized_data = std::str::from_utf8(&dec).unwrap();
                wallet.self_save_to_json("ed25519-secure_cell");
                println!("✅ aes256 hash is valid");
                return (true, deserialized_data.to_string());

            },
            Err(e) => (false, String::from(""))
        }

    }

    //---------------------------------------------
    //----------- aes256ctr_poly1305aes -----------
    //---------------------------------------------
    pub fn ed25519_aes256_signing(data: &str, mut wallet: Wallet) -> String{

        // note that nonce must be unique per each user or a unique identity
        let mut default_aes256_config = &mut Aes256Config::default();
        default_aes256_config.secret_key = gen_random_chars(64); /*** ---- secret key must be 64 bytes or 512 bits */
        default_aes256_config.nonce = gen_random_chars(16); /*** ---- secret key must be 16 bytes or 128 bits */
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

                    wallet.self_save_to_json("ed25519-aes256");
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

    //------------------------------
    //----------- themis -----------
    //------------------------------
    pub fn ed25519_secure_cell_signing(data: &str, mut wallet: Wallet) -> String{

        let mut default_secure_cell_config = &mut SecureCellConfig::default();
        // following secret key is the sha3 keccak256 hash of random chars
        default_secure_cell_config.secret_key = {
            hex::encode(
                wallet.self_generate_keccak256_hash_from(
                    &gen_random_chars(64)
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

                    wallet.self_save_to_json("ed25519-secure_cell");
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
    |          EdDSA Ed25519 USING KECCAK256 SIGNING
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

                wallet.self_save_to_json("ed25519-keccak256");
                return base58_sig.unwrap();

            },
            Err(e) => return String::from("")
        }

    }
    
}