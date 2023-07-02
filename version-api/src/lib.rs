use kv::create_version_info_key_from_request;
use macros::return_if_error;
use prost::Message;
use protos::{
    encode_message_to_bytes,
    version::{
        validate_get_version_download_info_request, validate_version_info_message,
        GetVersionDownloadInfoRequest, GetVersionDownloadInfoResponse, VersionInfo,
    },
};
use verifications::does_version_info_match_request;
use worker::{event, Context, Env, Request, Response, Result, Router};

mod kv;
mod verifications;

#[event(fetch)]
async fn main(_: Request, _: Env, _: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", |_, _| async move {
            Response::ok("Please download the Buri CLI. This url does not host a website.")
        })
        .get_async("/getVersionDownloadInfo", |mut req, ctx| async move {
            let data = req.bytes().await?;
            let request = return_if_error!(
                GetVersionDownloadInfoRequest::decode(data.as_slice()),
                Response::error("Bad request", 400)
            );
            return_if_error!(
                validate_get_version_download_info_request(&request),
                Response::error("Bad request", 400)
            );
            let key = create_version_info_key_from_request(&request);
            let version_kv = ctx.kv("VERSIONS")?;
            let version_info_bytes = return_if_error!(
                version_kv.get(&key).bytes().await?.ok_or("Not found"),
                Response::error("Not found", 404)
            );
            let version_info = return_if_error!(
                VersionInfo::decode(version_info_bytes.as_slice()),
                Response::error("Internal error", 500)
            );
            return_if_error!(
                validate_version_info_message(&version_info),
                Response::error("Internal error", 500)
            );
            if !does_version_info_match_request(&request, &version_info) {
                return Response::error("Internal error", 500);
            }
            let response = GetVersionDownloadInfoResponse {
                download_urls: version_info.download_urls,
                checksum: Some(version_info.checksums[0].clone()),
            };
            Response::from_bytes(encode_message_to_bytes(&response))
        });
    Response::ok("I can update this with just a push!")
}
