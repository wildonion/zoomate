

use std::collections::HashMap;
use std::io::Write;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use actix_web::web;
use serde::{Serialize, Deserialize};
use futures_util::TryStreamExt;
use futures_util::StreamExt;
use tokio::io::AsyncWrite;

pub const TOO_LARGE_FILE_SIZE: &str = "File Is Too Large";
pub static UNSUPPORTED_FILE_TYPE: &str = "File Type Is Not Supported, Only [.png, .jpg, .pdf, .mp4, .mp3, .gif or .jpeg]";
pub type PanelHttpResponse = Result<actix_web::HttpResponse, actix_web::Error>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<'m, T>{
    pub data: Option<T>,
    pub message: &'m str, // &str are a slice of String thus they're behind a pointer and every pointer needs a valid lifetime which is 'm in here 
    pub status: u16,
    pub is_error: bool
}

// we can represent any type of data as a serde json value
pub async fn multipart(
    payload: std::sync::Arc<tokio::sync::Mutex<Multipart>>
) -> Result<(serde_json::Value, HashMap<String, Vec<u8>>), PanelHttpResponse>{

    /* ------------------------------------------------------------------------
        streaming over each field of Multipart to extract utf8 bytes of each 
        text or file field value to create a map between between text or file
        fields and their values, we can return the map between text fields and 
        their values as a json value so later on convert it into an structure
        to build an instance of it and mutate those fields in server.
    */
    let mut text_fields: HashMap<String, String> = HashMap::new();
    let mut file_fields: HashMap<String, Vec<u8>> = HashMap::new();
    let lock_payload = payload.lock().await;
    let mut payload = lock_payload;
    let mut file_buffer = vec![];

    while let Ok(Some(mut field)) = payload.try_next().await {

        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or_default().to_string();

        /* extracting text fields */
        if let None = content_disposition.get_filename(){
            
            let data = field.next().await.unwrap_or(Ok(bytes::Bytes::from(""))).unwrap();
            text_fields.insert(field_name, std::str::from_utf8(&data).unwrap().to_string());
        
        } else{

            /* extracting file fields */
            let filename = content_disposition.get_filename().unwrap_or_default().to_lowercase();
            
            /* 
                receiving asyncly by streaming over the field future io object,
                getting the some part of the next field future object to extract 
                the image bytes from it, we can also use the following syntax to
                gather all bytes into a single buffer:
                let buffer = field.try_next().await.unwrap().unwrap().to_vec();

            */
            while let Some(chunk) = field.next().await{
                
                /* chunk is a Bytes object that can be used to be written into a buffer */
                let data = chunk.unwrap();

                /* 
                    getting the size of the file, data can be coerced 
                    to &[u8] by taking a reference to the underlying data
                */
                file_buffer.extend_from_slice(&data);
                
            }

            /* if the file size was greater than 200 MB reject the request */
            if file_buffer.len() > std::env::var("FILE_SIZE").unwrap().parse::<usize>().unwrap(){

                /* terminate the method and respond the caller */
                let resp = Response::<&[u8]>{
                    data: Some(&[]),
                    message: TOO_LARGE_FILE_SIZE,
                    status: 406,
                    is_error: true
                };
                return Err(
                    Ok(HttpResponse::NotAcceptable().json(resp))
                );
            }

            file_fields.insert(filename, file_buffer.clone());

        }
    }

    let json_data = serde_json::to_value(&text_fields).unwrap();

    Ok(
        (json_data, file_fields)
    )

}