#[cfg(feature = "client")]
use reqwest::{header, Client, Response};

/// An HTTP method abstraction
#[derive(Debug, Copy, Clone)]
pub enum Method {
    Get,
    Post,
}

/// An HTTP content type abstraction
#[derive(Debug, Copy, Clone)]
pub enum ContentType {
    FormUrlEncoded,
    Json,
}

/// An HTTP user agent abstraction
#[derive(Debug, Clone)]
pub struct UserAgent(pub String);

/// An abstraction for basic http request.
#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub body: Option<String>,
    pub content_type: Option<ContentType>,
    pub user_agent: Option<UserAgent>,
    pub method: Method,
}

#[cfg(feature = "client")]
impl Request {
    pub fn execute(&self, client: &Client) -> Option<Response> {
        let builder = match self.method {
            Method::Get => client.get(&self.url),
            Method::Post => client.post(&self.url),
        };

        let builder = if let Some(agent) = self.user_agent.clone() {
            builder.header(header::USER_AGENT, agent.0)
        } else {
            builder
        };

        let builder = if let Some(content_type) = self.content_type.clone() {
            match content_type {
                ContentType::Json => {
                    builder.header(header::CONTENT_TYPE, String::from("application/json"))
                }
                ContentType::FormUrlEncoded => builder.header(
                    header::CONTENT_TYPE,
                    String::from("application/www-form-url-encoded"),
                ),
            }
        } else {
            builder
        };

        let builder = if let Some(body) = self.body.clone() {
            builder.body(body)
        } else {
            builder
        };

        builder.send().ok()
    }
}
