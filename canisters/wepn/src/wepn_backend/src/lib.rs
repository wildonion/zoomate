



/* 

    actor and onchain based pubsub realtime streaming contract to build wepn using 
    actix,redis,grpc,tcp,ws stream handler,mpsc,libp2p,ipfs,tor on ICP with 
    wallexerr::ed25519_aes_signing,tokio::tcp,mpsc,spawn,time,select,mutex,
    asynciotraits,while let Some
    bypass all DPIs

*/

pub struct Connected;
pub struct Disconnected;


#[ic_cdk::query]
fn start_wepn_actor(name: String) -> Result<Connected, Disconnected> {
    
    Ok(
        Connected
    )
}
