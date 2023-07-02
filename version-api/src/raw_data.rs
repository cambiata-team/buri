#[cfg(test)]
/// A real response from GitHub about a release. This information is publicly
/// available and can be committed into the repo.
/// https://api.github.com/repos/cambiata-team/buri/releases/110756292
pub const RAW_RELEASE_DATA: &str = r#"{
    "url": "https://api.github.com/repos/cambiata-team/buri/releases/110756292",
    "assets_url": "https://api.github.com/repos/cambiata-team/buri/releases/110756292/assets",
    "upload_url": "https://uploads.github.com/repos/cambiata-team/buri/releases/110756292/assets{?name,label}",
    "html_url": "https://github.com/cambiata-team/buri/releases/tag/0.2.0",
    "id": 110756292,
    "author": {
      "login": "github-actions[bot]",
      "id": 41898282,
      "node_id": "MDM6Qm90NDE4OTgyODI=",
      "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
      "gravatar_id": "",
      "url": "https://api.github.com/users/github-actions%5Bbot%5D",
      "html_url": "https://github.com/apps/github-actions",
      "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
      "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
      "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
      "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
      "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
      "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
      "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
      "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
      "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
      "type": "Bot",
      "site_admin": false
    },
    "node_id": "RE_kwDOJz5JPs4GmgHE",
    "tag_name": "0.2.0",
    "target_commitish": "main",
    "name": "0.2.0",
    "draft": false,
    "prerelease": false,
    "created_at": "2023-07-01T22:42:53Z",
    "published_at": "2023-07-01T22:43:42Z",
    "assets": [
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185498",
        "id": 115185498,
        "node_id": "RA_kwDOJz5JPs4G3Zda",
        "name": "cli-aarch64-apple-darwin.sha256",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/octet-stream",
        "state": "uploaded",
        "size": 98,
        "download_count": 0,
        "created_at": "2023-07-01T22:45:00Z",
        "updated_at": "2023-07-01T22:45:00Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.sha256"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185499",
        "id": 115185499,
        "node_id": "RA_kwDOJz5JPs4G3Zdb",
        "name": "cli-aarch64-apple-darwin.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/x-gtar",
        "state": "uploaded",
        "size": 515264,
        "download_count": 2,
        "created_at": "2023-07-01T22:45:00Z",
        "updated_at": "2023-07-01T22:45:00Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-apple-darwin.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185603",
        "id": 115185603,
        "node_id": "RA_kwDOJz5JPs4G3ZfD",
        "name": "cli-aarch64-unknown-linux-gnu.sha256",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/octet-stream",
        "state": "uploaded",
        "size": 103,
        "download_count": 0,
        "created_at": "2023-07-01T22:46:39Z",
        "updated_at": "2023-07-01T22:46:39Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-unknown-linux-gnu.sha256"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185604",
        "id": 115185604,
        "node_id": "RA_kwDOJz5JPs4G3ZfE",
        "name": "cli-aarch64-unknown-linux-gnu.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/x-gtar",
        "state": "uploaded",
        "size": 1366150,
        "download_count": 0,
        "created_at": "2023-07-01T22:46:39Z",
        "updated_at": "2023-07-01T22:46:40Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-aarch64-unknown-linux-gnu.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185553",
        "id": 115185553,
        "node_id": "RA_kwDOJz5JPs4G3ZeR",
        "name": "cli-universal-apple-darwin.sha256",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/octet-stream",
        "state": "uploaded",
        "size": 100,
        "download_count": 0,
        "created_at": "2023-07-01T22:45:44Z",
        "updated_at": "2023-07-01T22:45:44Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-universal-apple-darwin.sha256"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185554",
        "id": 115185554,
        "node_id": "RA_kwDOJz5JPs4G3ZeS",
        "name": "cli-universal-apple-darwin.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/x-gtar",
        "state": "uploaded",
        "size": 1031013,
        "download_count": 0,
        "created_at": "2023-07-01T22:45:44Z",
        "updated_at": "2023-07-01T22:45:45Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-universal-apple-darwin.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185504",
        "id": 115185504,
        "node_id": "RA_kwDOJz5JPs4G3Zdg",
        "name": "cli-x86_64-apple-darwin.sha256",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/octet-stream",
        "state": "uploaded",
        "size": 97,
        "download_count": 0,
        "created_at": "2023-07-01T22:45:04Z",
        "updated_at": "2023-07-01T22:45:04Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-x86_64-apple-darwin.sha256"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185503",
        "id": 115185503,
        "node_id": "RA_kwDOJz5JPs4G3Zdf",
        "name": "cli-x86_64-apple-darwin.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/x-gtar",
        "state": "uploaded",
        "size": 403915,
        "download_count": 0,
        "created_at": "2023-07-01T22:45:04Z",
        "updated_at": "2023-07-01T22:45:04Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-x86_64-apple-darwin.tar.gz"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185494",
        "id": 115185494,
        "node_id": "RA_kwDOJz5JPs4G3ZdW",
        "name": "cli-x86_64-unknown-linux-gnu.sha256",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/octet-stream",
        "state": "uploaded",
        "size": 102,
        "download_count": 0,
        "created_at": "2023-07-01T22:44:43Z",
        "updated_at": "2023-07-01T22:44:44Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-x86_64-unknown-linux-gnu.sha256"
      },
      {
        "url": "https://api.github.com/repos/cambiata-team/buri/releases/assets/115185493",
        "id": 115185493,
        "node_id": "RA_kwDOJz5JPs4G3ZdV",
        "name": "cli-x86_64-unknown-linux-gnu.tar.gz",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "application/x-gtar",
        "state": "uploaded",
        "size": 444747,
        "download_count": 0,
        "created_at": "2023-07-01T22:44:43Z",
        "updated_at": "2023-07-01T22:44:44Z",
        "browser_download_url": "https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-x86_64-unknown-linux-gnu.tar.gz"
      }
    ],
    "tarball_url": "https://api.github.com/repos/cambiata-team/buri/tarball/0.2.0",
    "zipball_url": "https://api.github.com/repos/cambiata-team/buri/zipball/0.2.0",
    "body": null
}"#;

#[cfg(test)]
/// The SHA256 checksum of the `cli-x86_64-apple-darwin.tar.gz` file.
/// https://github.com/cambiata-team/buri/releases/download/0.2.0/cli-x86_64-apple-darwin.sha256
pub const SHA_256_FILE: &str = "dc06fa9d945ac660dd095a92032cc207cdf94f8864d074cb83d9e1606f5f5a0b  cli-x86_64-apple-darwin.tar.gz";
