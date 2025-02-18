use crate::exports::edgee::protocols::data_collection::Dict;
use anyhow::Context;
use aws_credential_types::Credentials;
use aws_sigv4::http_request::{
    sign, SignableBody, SignableRequest, SigningParams, SigningSettings,
};
use aws_sigv4::sign::v4;
use aws_smithy_runtime_api::client::identity::Identity;
use base64::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct FirehoseRequestRecord {
    pub data: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct FirehoseRequestBody {
    pub record: FirehoseRequestRecord,
    pub delivery_stream_name: String,
}

pub struct Settings {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: String, // could be empty
    pub region: String,
    pub firehose_stream: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let access_key = settings_map
            .get("aws_access_key")
            .context("Missing AWS Access Key")?
            .to_string();

        let secret_key = settings_map
            .get("aws_secret_key")
            .context("Missing AWS Secret Key")?
            .to_string();

        let session_token = settings_map
            .get("aws_session_token")
            .map(String::to_string)
            .unwrap_or_default(); // optional

        let region = settings_map
            .get("aws_region")
            .context("Missing AWS region")?
            .to_string();

        let firehose_stream = settings_map
            .get("firehose_stream")
            .context("Missing stream name")?
            .to_string();

        Ok(Self {
            access_key,
            secret_key,
            session_token,
            region,
            firehose_stream,
        })
    }

    pub fn generate_firehose_url(&self) -> String {
        format!(
            "https://{}/", // that final / is very important!
            self.generate_firehose_host(),
        )
    }

    pub fn generate_firehose_host(&self) -> String {
        format!("firehose.{}.amazonaws.com", self.region.clone())
    }

    pub fn generate_body(&self, data: String) -> String {
        let delivery_stream_name = self.firehose_stream.clone();
        let record = FirehoseRequestRecord {
            data: BASE64_STANDARD.encode(data),
        };

        // create body obj and serialize to JSON string
        let body = FirehoseRequestBody {
            record,
            delivery_stream_name,
        };
        serde_json::to_string(&body).unwrap_or_default()
    }

    pub fn generate_headers(&self, url: String, body: String) -> Vec<(String, String)> {
        let session_token = if self.session_token.is_empty() {
            None
        } else {
            Some(self.session_token.clone())
        };

        // create Identity with static Credentials
        let identity: Identity = Credentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            session_token,
            None,
            "hardcoded-credentials",
        )
        .into();

        let signing_settings = SigningSettings::default();

        // build signing parameters
        let signing_params: SigningParams = v4::SigningParams::builder()
            .identity(&identity)
            .region(self.region.as_str())
            .name("firehose")
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .unwrap()
            .into();

        // create a signable request
        let signable_request = SignableRequest::new(
            "POST",
            url,
            std::iter::empty(),
            SignableBody::Bytes(body.as_bytes()),
        )
        .expect("signable request");

        // generate the signature headers
        let (signing_instructions, _signature) = sign(signable_request, &signing_params)
            .unwrap()
            .into_parts();

        // convert to Vec<(String, String)>
        let mut headers: Vec<(String, String)> = signing_instructions
            .headers()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        // these are required, but not needed for signing
        headers.extend(vec![
            (
                "x-amz-target".to_string(),
                "Firehose_20150804.PutRecord".to_string(),
            ),
            (
                "content-type".to_string(),
                "application/x-amz-json-1.1".to_string(),
            ),
        ]);

        headers
    }
}
