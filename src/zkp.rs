


use crate::*;


pub mod zkp{

    /* e2e and file and msg encryption using aes256 bits in wallexerr: see ed25519_aes256_test() test method

        tools: RSA ed25519 ECC curve with aes256 hash in wallexerr, openssl and ring for RSA + KDF like sha256 and keccak256
        ransomewere, steganography and files encryption to generate unique assets by encrypting using 
        aes256 cbc with pbkdf2 + sha384 + salt then showing key and iv like:
                    
            openssl aes-256-cbc -a -salt -pbkdf2 -in img.png -out img.png.enc -p
            openssl aes-256-cbc -d -a -pbkdf2 -in img.png.enc -out img.png.new -p 

            openssl aes-256-cbc -md sha384 -in secret.txt -out img.png.enc -p
            openssl aes-256-cbc -d -nopad -md sha384 -in img.png.enc -p

            gpg --output encrypted.data --symmetric --cipher-algo AES256 un_encrypted.data
            gpg --output un_encrypted.data --decrypt encrypted.data
    
    */

    pub use super::*;

    pub struct ZkpError;
    pub struct Verifier;
    pub struct Prover;

    pub async fn auth() -> Result<(), ZkpError>{

        Ok(())
    }

    fn get_zkp_comparator() -> themis::secure_comparator::SecureComparator{
        wallexerr::misc::Wallet::generate_zkp_comparator()
    }

    // https://noir-lang.org/index.html
    // https://github.com/rust-cc/awesome-cryptography-rust#zero-knowledge-proofs
    // https://github.com/cossacklabs/themis/blob/master/docs/examples/rust/secure_compare.rs => themis zkp

}