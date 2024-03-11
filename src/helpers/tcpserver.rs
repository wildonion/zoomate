


use oauth2::helpers;
use ring::signature;
use tokio::io::AsyncReadExt; // for reading from socket asyncly allows us to call .read() method
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt; // for writing to socket asyncly allows us to call .write_all() method
use log::{info, error};
use wallexerr::misc::SecureCellConfig;
use crate::constants::SECURECELLCONFIG_TCPWALLET;
use crate::constants::gen_random_chars;
use crate::*;
use crate::helpers::*;


/*   -------------------------- STREAMING NOTES --------------------------
    |   streaming can be done using actix|tokio|tonic with tlps like wsactor|http|tcp|grpc in a separate 
    |   threadpool like in tokio::spawn(), actors in a same server can use actix and tokio stuffs to send/recv 
    |   responses actors in two different mses can use tcp, (g)capnprpc or redis to send/recv responses also 
    |   there must be a message and stream handlers implemented for actors so they can communicate with each 
    |   other and different parts of the app to send/receive static lazy mutex streams of utf8 bytes data based 
    |   on serde_json, web::Payload, Multipart and capnp+protobuf codecs throught rpc or mpsc channel based on 
    |   tokio::spawn,mpsc,mailbox,mutex,select,time, we can also have a pubsub pattern for them using 
    |   libp2pgossipsub,rpc,redisstreamqueue,actixbroker pubsub

        also see extractor::multipart() which handles incoming multipart form data asyncly 
        by streaming over each field to gather the field's bytes then map it into a 
        data type or serde json value
        
        streaming over a realtiming source like a socket to fill the buffer with incoming u8 
        future byte objs chunks and then map into a struct can be done with tokio(mpsc,select,spawn,
        mutex,rwlock,tcp) actix-ws-http,redis&libp2ppubsub and can be a webhook/stream/event 
        handler which will accept streaming of events' data utf8 bytes can be like:  

        let (data_sender, mut data_receiver) 
            = tokio::sync::mpsc::channel::<std::sync::Arc<tokio::sync::Mutex<Data>>>(1024);
        let buffer = vec![];
        let mut bytes = web::BytesMut::new();
        let streamer_body: web::Payload;
        tokio::task::spawn(async move{ 
            while let Some(chunk) = streamer_body.next().await{
                let byte = chunk.as_slice();
                buffer.extend_from_slice(byte);
                bytes.extend_from_slice(byte);
                let decoded_data = serde_json::from_slice::<Data>(&buffer).unwrap();
                data_sender.clone().send(
                    std::sync::Arc::new(
                        tokio::sync::Mutex::new(
                            Some(decoded_data)
                        )
                    )
                ).await;
            }
        });
        while let Some(received_data) = data_receiver.recv().await{
            let mut data = received_data.lock().await;
            *data = Default::default();
        }

        the nature of rust codes are not asynced and multithreaded by default we must use
        a runtime for that like tokio and run async tasks inside tokio::spawn() threadpool
        which takes care of running an async context in a free thread behind the scene and 
        won't let other codes in other scopes get halted and waited for this job to be
        finished and, they get exectued on their own without blocking the scopes  
        thus if we have a condition like
        if condition {
            return something to the caller;
        }

        the rest of the code after if won't get executed with this nature we can 
        only have one if, provided that it terminate the method body with an statement,
        and respond the caller with a value; once the body get terminated the rest of
        the code won't be executed cause we don't have async context by default, 
        other than that we have to provide the else part since rust needs to know 
        that if not this type then what type?!
*/
pub struct TcpListenerActor{
    pub addr: String,
    pub wallet: wallexerr::misc::Wallet,
    pub secure_cell: wallexerr::misc::SecureCellConfig
}

impl TcpListenerActor{

    pub fn new(
        wallet: wallexerr::misc::Wallet, 
        secure_cell: wallexerr::misc::SecureCellConfig,
        address: &str,
    ) -> Self{
        TcpListenerActor{
            wallet,
            secure_cell,
            addr: address.to_string()
        }
    }

    pub async fn start_streaming(&self){

        // extracting self, since self is behind a mutable pointer 
        // thus all the fields will be extract in form of mutable pointer

        let addr = self.addr.clone();
        let api_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        let (job_sender, mut job_receiver) = 
            tokio::sync::mpsc::channel::<String>(1024);

        ////// cloning before going into first tokio::spawn scope
        // getting the shared tcp ed25519 secure cell config and wallet
        let mut secure_cell = self.secure_cell.clone();
        let wallet = self.wallet.clone();
        let cloned_job_sender = job_sender.clone();
        
        /* 
            once we run this method and get to here, this tokio::spawn contains 
            the whole streaming logic which will be executed in the background 
            hence allows the code to be finished executing and have no constant 
            listening in the terminal, the solution to this is to make the app 
            alive and don't let it to be finished so we can monitor the process
            inside this tokio::spawn while the app is running, this can be done
            by adding a loop{} after calling the start_streaming() method which
            allows the app to be ran constantly and prevent finishing execution
        */
        tokio::spawn(async move{

            ////// cloning before going into while loop scope
            let cloned_aes256_config = secure_cell.clone();

            info!("🚀 tcp listener is started at [{}] to accept streaming of utf8 bytes", "0.0.0.0:2247");

            // streaming over incoming bytes to fill the buffer and then map the buffer to structure
            while let Ok((mut api_streamer, addr)) = api_listener.accept().await{ // execute the accepting process of a tcp listener asyncly inside a tokio threadpool

                info!("🍐 new peer connection: [{}]", addr);

                ////// cloning before going into second tokio::spawn scope
                let mut cloned_aes256_config = cloned_aes256_config.clone();
                let mut cloned_wallet = wallet.clone();
                let cloned_job_sender = cloned_job_sender.clone();

                tokio::spawn(async move { // execute the reading process from the socket stream asyncly inside a tokio threadpool

                    /* this buffer will be filled up with incoming bytes from the socket */
                    let mut buffer = vec![]; // or vec![0u8; 1024] // filling all the 1024 bytes with 0

                    while match api_streamer.read(&mut buffer).await { /* streaming over socket to fill the buffer */
                        Ok(rcvd_bytes) if rcvd_bytes == 0 => return,
                        Ok(rcvd_bytes) => {  

                            let aes256_config = &mut cloned_aes256_config;
                            let string_event_data = std::str::from_utf8(&buffer[..rcvd_bytes]).unwrap(); // map the fulfilled buffer into str
                            info!("📺 received event data from peer: {}", string_event_data);
                            
                            // parsing the signature and hash of data
                            let mut splitted_string_event_data = string_event_data.split("|");
                            let signature = splitted_string_event_data.next().unwrap();
                            let hash_data = splitted_string_event_data.next().unwrap();
                            aes256_config.data = hash_data.as_bytes().to_vec(); // filling it with the hash of data for verifying and decrypting

                            /* ------------------------------------------------------------------------------------------- */
                            /* -------- verifying and decrypting the tcp packet using ed25519 with aes256 signing -------- */
                            /* ------------------------------------------------------------------------------------------- */
                            let (is_verified, decrypted_data) = cry::eddsa_with_symmetric_signing::ed25519_decrypt_and_verify_tcp_packet_with_aes256_secure_cell(cloned_wallet.clone(), signature, aes256_config);
                            let must_be_encrypted = if is_verified{
                                info!("✅ decrypted aes256 hash data from client is => {:?}", decrypted_data);
                                /* ----------------------------------------------------------------------------- */
                                /* -------- encrypting the tcp packet using ed25519 with aes256 signing -------- */
                                /* ----------------------------------------------------------------------------- */
                                aes256_config.data = String::from("****a very important event data****").as_bytes().to_vec(); // filling it with the raw data for signing and encrypting
                                // client must verify the signature using the hash of data and public key
                                let sig = cry::eddsa_with_symmetric_signing::ed25519_encrypt_and_sign_tcp_packet_with_aes256_secure_cell(cloned_wallet.clone(), aes256_config);
                                let hash_of_data = aes256_config.clone().data; // data field now contains the hash of data
                                let sig_and_hash_data = format!("{}|{}", sig, hex::encode(hash_of_data));
                                /* ----------------------------------------------------------------------------- */
                                /* ----------------------------------------------------------------------------- */
                                /* ----------------------------------------------------------------------------- */
                                // sending the signature and hash data through the socket back to client so 
                                // it can verify the connection and if it was verified then 
                                // client can send encrypted packet through the secure connection
                                sig_and_hash_data
                            } else{
                                String::from("❌ invalid hash data or signature, connection is not secured")
                            };
                            
                            /*  
                                sending the decoded bytes into the mpsc channel so we could receive it  
                                in other scopes or threads
                            */
                            if let Err(why) = cloned_job_sender.send(must_be_encrypted.clone()).await{
                                error!("❌ failed to send to the mpsc channel; {}", why);
                            }

                            // encrypting the signature and the hash of data using aes256 config
                            // later on client must decrypt this data to extract the signature 
                            // and the hash of data to start verification process.
                            let pointer_to_secure_cell = &mut cloned_aes256_config.clone();
                            pointer_to_secure_cell.data = must_be_encrypted.as_bytes().to_vec();
                            let fully_encrypted_message_bytes = cloned_wallet.self_secure_cell_encrypt(pointer_to_secure_cell).unwrap();
                            let fully_encrypted_message = std::str::from_utf8(&fully_encrypted_message_bytes).unwrap();

                            // writing the data into the socket 
                            if let Err(why) = api_streamer.write_all(&fully_encrypted_message.as_bytes()).await{
                                error!("❌ failed to write data to api_streamer; {}", why);
                                return;
                            } else{
                                info!("🗃️ sent {}, wrote {} bytes to api_streamer", string_event_data, string_event_data.len());
                                return;
                            }
                        
                        },
                        Err(e) => {
                            error!("❌ failed to read from api_streamer; {:?}", e);
                            return;
                        }
                        
                    }{} // this belongs to the while match
            
                });
            }
        });


        // receiving the data from the channel using while let some 
        // and tokio select event loop in the background also 
        // the receiver of the tokio event loop will be executed 
        // first to receive the data from the sender, cause we've
        // sent some data right after this tokio::spawn 
        tokio::spawn(async move{
            // running a loop inside tokio::spawn() to receive constantly
            // from the mpsc channel it's like having while let Some
            loop{

                // ----------- tokio event loop
                // ---------------------------------------------------------------
                tokio::select!{
                    received_job = job_receiver.recv() => {
                        if let Some(job) = received_job{
                            info!("tokio::select > got the job: {:?}", job);
                        } else{
                            break;
                        }
                    }
                }
                
            }
            // ----------- while let some instead of loop{tokio::select!{}}
            // ---------------------------------------------------------------
            while let Some(job) = job_receiver.recv().await{
                info!("while let some > got the job: {:?}", job);
            }
            
        });
    
        job_sender
            .clone()
            .send(String::from("this data has sent from the bottom of start_streaming method"))
            .await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        job_sender
            .clone()
            .send(String::from("another data sent from the bottom of start_streaming method"))
            .await.unwrap();



    }

}