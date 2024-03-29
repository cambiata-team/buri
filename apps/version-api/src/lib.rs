use kv::{create_version_info, create_version_info_key_from_request, get_keys_from_binary_info};
use macros::return_if_error;
use parse_release::{
    get_hash_from_sha256_file, parse_asset_to_binary_info, BinaryInfo, Release, WebhookBody,
};
use prost::Message;
use protos::{
    decode_base_64_to_bytes, encode_message_to_base_64,
    version::{
        validate_get_version_download_info_request, validate_version_info_message,
        version_info_key::Version, Architecture, Channel, GetVersionDownloadInfoRequest,
        GetVersionDownloadInfoResponse, OperatingSystemFamily, Program, VersionInfo,
        VersionInfoKey,
    },
};
use verifications::does_version_info_match_request;
use version::{is_valid_version, normalize_version};
use worker::{event, Cache, Context, Env, Fetch, Method, Request, Response, Result, Router};

mod kv;
mod parse_release;
mod raw_data;
mod verifications;

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok("Please download the Buri CLI. This url does not host a website.")
        })
        .get_async("/get-version-download-info", |req, ctx| async move {
            let cache = Cache::default();
            let url = req.url()?;
            let cache_key = url.to_string();
            // let cached_response = cache.get(&cache_key, false).await?;
            // if let Some(cached_response) = cached_response {
            //     return Ok(cached_response);
            // }
            let query = &return_if_error!(
                url.query().ok_or("no query parameters"),
                Response::error("Bad request: no query parameters", 400)
            )[2..]; // Remove the leading "?q=".
            let data = decode_base_64_to_bytes(query);
            let request = return_if_error!(
                GetVersionDownloadInfoRequest::decode(data.as_slice()),
                Response::error("Bad request: cannot decode request proto", 400)
            );
            if let Err(error) = validate_get_version_download_info_request(&request) {
                return Response::error(
                    format!("Bad request: invalid version download info request - {error:?}"),
                    400,
                );
            }
            let key = create_version_info_key_from_request(&request);
            let version_kv = ctx.kv("VERSIONS")?;
            let version_info_base64 = return_if_error!(
                version_kv.get(&key).text().await?.ok_or("Not found"),
                Response::error("Not found", 404)
            );
            let version_info_bytes = decode_base_64_to_bytes(&version_info_base64);
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
                version_number: version_info.version_number,
            };
            let mut response = Response::ok(encode_message_to_base_64(&response))?;
            // Cache for 1 hour. In theory, the version info should rarely change per quest.
            // However setting it to one hour allows us to update the version info when
            // there should be a change (e.g., yank an insecure version, add new download URLs, etc.).
            // By using the cache API Cloudflare will cache it globally, not just from the one
            // data center.
            response
                .headers_mut()
                .set("cache-control", "s-maxage=3600")?;
            cache.put(&cache_key, response.cloned()?).await?;
            Ok(response)
        })
        .post_async("/add-release", |mut req, ctx| async move {
            let auth_header = req.headers().get("Authentication")?;
            match auth_header {
                Some(auth_header) => {
                    if auth_header != ctx.secret("RELEASE_WEBHOOK_SECRET")?.to_string() {
                        return Response::error("Unauthorized", 401);
                    }
                }
                _ => {
                    return Response::error("Unauthorized", 401);
                }
            }
            let body = req.text().await?;
            let release_id = return_if_error!(
                serde_json::from_str::<WebhookBody>(&body),
                Response::error(format!("Bad request - cannot parse body: {body}"), 400)
            )
            .release_id;
            let raw_release_data = fetch(&format!(
                "https://api.github.com/repos/cambiata-team/buri/releases/{release_id}"
            ))
            .await?;
            let release_data: Release = return_if_error!(
                serde_json::from_str(&raw_release_data),
                Response::error(
                    format!("Bad request - cannot parse release data: {raw_release_data}"),
                    400
                )
            );
            let version = normalize_version(release_data.tag_name);
            if !is_valid_version(version) {
                return Response::error(
                    "Bad request - version is not specified or is invalid",
                    400,
                );
            }
            let binary_infos: Vec<BinaryInfo> = release_data
                .assets
                .iter()
                .filter_map(parse_asset_to_binary_info)
                .collect();
            for info in binary_infos {
                let sha_file = fetch(&info.hash_download_url).await?;
                let sha256 = get_hash_from_sha256_file(&sha_file);
                let version_info = create_version_info(&info, version, sha256);
                let (latest_key, version_key) = get_keys_from_binary_info(&info, version);

                // Ensure that we only save valid version info into the KV store.
                return_if_error!(validate_version_info_message(&version_info), {
                    Response::error("Internal error", 500)
                });

                let version_kv = ctx.kv("VERSIONS")?;
                version_kv
                    .put(&latest_key, &encode_message_to_base_64(&version_info))?
                    .execute()
                    .await?;
                version_kv
                    .put(&version_key, &encode_message_to_base_64(&version_info))?
                    .execute()
                    .await?;
            }
            Response::ok("Added")
        })
        .get_async("/get-latest-cli-version-plaintext", |_, ctx| async move {
            let mut key = VersionInfoKey::default();
            // Hard code the defaults since presumably all builds will have the same version.
            key.set_program(Program::VersionManager);
            key.set_operating_system_family(OperatingSystemFamily::Darwin);
            key.set_architecture(Architecture::Arm64);
            key.version = Some(Version::Channel(Channel::Latest.into()));

            let version_kv = ctx.kv("VERSIONS")?;
            let encoded_key = encode_message_to_base_64(&key);
            let version_info_base64 = return_if_error!(
                version_kv
                    .get(&encoded_key)
                    .text()
                    .await?
                    .ok_or("Not found"),
                Response::error("Not found", 404)
            );
            let version_info_bytes = decode_base_64_to_bytes(&version_info_base64);
            let version_info = return_if_error!(
                VersionInfo::decode(version_info_bytes.as_slice()),
                Response::error("Internal error", 500)
            );
            return_if_error!(
                validate_version_info_message(&version_info),
                Response::error("Internal error", 500)
            );

            Response::ok(version_info.version_number)
        })
        .or_else_any_method("/*catchall", |_, _| {
            Response::error("This page does not exist", 404)
        })
        .run(req, env)
        .await
}

async fn fetch(url: &str) -> Result<String> {
    let mut request = Request::new(url, Method::Get)?;
    // See https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
    request.headers_mut()?.set("User-Agent", "cambiata-team")?;
    let body = Fetch::Request(request).send().await?.text().await?;
    Ok(body)
}
