#![feature(custom_attribute)]
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod model;

use model::{Webhook, ExecuteData};

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper::Url;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper_native_tls::NativeTlsClient;

const DISCORD_API_BASE_URL: &str = "https://discordapp.com/api/";

pub struct DiscordWebhookApi {
    client: Client,
}

impl DiscordWebhookApi {
    pub fn new() -> DiscordWebhookApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        DiscordWebhookApi {
            client: Client::with_connector(connector),
        }
    }

    pub fn execute_webhook(&self, webhook: &Webhook, data: ExecuteData) {
        let mut url = Url::parse(DISCORD_API_BASE_URL).unwrap();
        url = url.join(&format!("webhooks/{id}/{token}", id = webhook.id, token = webhook.token)).unwrap();

        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

        let payload = serde_json::to_string(&data).unwrap();

        self.client
            .post(url)
            .headers(headers)
            .body(&payload)
            .send()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
