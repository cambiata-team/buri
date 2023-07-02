use kv::create_version_info_key_from_request;
use macros::return_if_error;
use prost::Message;
use protos::{
    decode_base_64_to_bytes, encode_message_to_bytes,
    version::{
        validate_get_version_download_info_request, validate_version_info_message,
        GetVersionDownloadInfoRequest, GetVersionDownloadInfoResponse, VersionInfo,
    },
};
use verifications::does_version_info_match_request;
use worker::{event, Cache, Context, Env, Request, Response, Result, Router};

mod kv;
mod verifications;

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok("Please download the Buri CLI. This url does not host a website.")
        })
        .get_async("/getVersionDownloadInfo", |req, ctx| async move {
            let cache = Cache::default();
            let url = req.url()?;
            let cache_key = url.to_string();
            let cached_response = cache.get(&cache_key, false).await?;
            if let Some(cached_response) = cached_response {
                return Ok(cached_response);
            }
            let query = &return_if_error!(
                url.query().ok_or("no query parameters"),
                Response::error("Bad request", 400)
            )[2..]; // Remove the leading "?q=".
            let data = decode_base_64_to_bytes(query);
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
            let mut response = Response::from_bytes(encode_message_to_bytes(&response))?;
            // Cache for 1 hour.
            response
                .headers_mut()
                .set("cache-control", "s-maxage=3600")?;
            let cache_key = url.to_string();
            cache.put(&cache_key, response.cloned()?).await?;
            Ok(response)
        })
        .or_else_any_method("/*catchall", |_, _| {
            Response::error("This page does not exist", 404)
        })
        .run(req, env)
        .await
}
