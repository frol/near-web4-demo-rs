use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
pub struct Contract;

#[near]
impl Contract {
    /// Learn more about web4 here: https://web4.near.page
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        if request.path == "/" {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: "<h1>Hello from Web4 on NEAR!</h1>"
                    .as_bytes()
                    .to_owned()
                    .into(),
            }
        } else {
            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: format!("<h1>Some page</h1><pre>{:#?}</pre>", request)
                    .into_bytes()
                    .into(),
            }
        }
    }
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub query: std::collections::HashMap<String, Vec<String>>,
    pub preloads: Option<std::collections::HashMap<String, Web4Response>>,
}

#[near(serializers = [json])]
#[derive(Debug)]
#[serde(untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: near_sdk::json_types::Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
}
