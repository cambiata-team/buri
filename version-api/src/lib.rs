use prost::Message;
use protos::{
    encode_message_to_bytes,
    version::{GetVersionDownloadInfoRequest, GetVersionDownloadInfoResponse},
};
use worker::{event, Context, Env, Request, Response, Result, Router};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", |_, _| async move {
            Response::ok("Please download the Buri CLI. This url does not host a website.")
        })
        .get_async("/getVersionDownloadInfo", |mut req, ctx| async move {
            let data = req.bytes().await?;
            let result = GetVersionDownloadInfoRequest::decode(data.as_slice());
            match result {
                Ok(request) => {
                    let response = GetVersionDownloadInfoResponse::default();
                    let program = request.program();
                    let response_size = request.encoded_len();
                    Response::from_bytes(encode_message_to_bytes(&response))
                }
                Err(e) => Response::error("Bad request", 400),
            }
        });
    Response::ok("I can update this with just a push!")
}
