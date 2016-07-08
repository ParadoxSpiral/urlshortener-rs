//! Library service providers implementation.

extern crate hyper;

use hyper::client::{Client, Response};
use hyper::header::ContentType;

/// Used to specify which provider to use to generate a short URL.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Provider {
    /// https://bam.bz provider
    BamBz,
    /// https://bn.gy provider
    BnGy,
    /// http://fifo.cc provider
    FifoCc,
    /// https://hec.su provider
    ///
    /// * Limited to 3000 API requests per day
    HecSu,
    /// https://is.gd provider
    IsGd,
    /// http://nowlinks.net provider
    NowLinks,
    /// http://phx.co.in provider
    /// After some time shows ads
    PhxCoIn,
    /// http://psbe.co provider
    PsbeCo,
    /// http://readbility.com provider
    Rdd,
    /// http://rlu.ru provider
    ///
    /// * Attention! If you send a lot of requests from one IP, it can be
    /// blocked. If you plan to add more then 100 URLs in one hour, please let
    /// the technical support know. Otherwise your IP can be blocked
    /// unexpectedly. Prior added URLs can be deleted.
    Rlu,
    /// http://tinyurl.com provider
    /// * Note: this service does not provide any API.
    /// The implementation result depends on the service result web page.
    TinyUrl,
    /// https://v.gd provider
    VGd,
}

impl Provider {
    /// Converts the Provider variant into its domain name equivilant
    pub fn to_name(&self) -> &str {
        match *self {
            Provider::BamBz => "bam.bz",
            Provider::BnGy => "bn.gy",
            Provider::FifoCc => "fifo.cc",
            Provider::HecSu => "hec.su",
            Provider::IsGd => "is.gd",
            Provider::NowLinks => "nowlinks.net",
            Provider::PhxCoIn => "phx.co.in",
            Provider::PsbeCo => "psbe.co",
            Provider::Rdd => "readability.com",
            Provider::Rlu => "rlu.ru",
            Provider::TinyUrl => "tinyurl.com",
            Provider::VGd => "v.gd",
        }
    }
}

/// Returns a vector of all `Provider` variants.
///
/// The providers which are discouraged from use due to limitations such as
/// rate limitations are at the end of the resultant vector.
pub fn providers() -> Vec<Provider> {
    vec![
        Provider::IsGd,
        Provider::BnGy,
        Provider::VGd,
        Provider::Rdd,
        Provider::BamBz,
        Provider::FifoCc,

        // The following list are items that are discouraged from use.
        // Reason: rate limit (100 requests per hour)
        Provider::Rlu,
        // Reason: rate limit (3000 requests per day)
        Provider::HecSu, 
        // Reason: does not provide an api
        Provider::TinyUrl,
        // Reason: unstable work
        Provider::PsbeCo,

        // The following list are items that show previews instead of direct links.
        Provider::NowLinks,

        // The following list are items that show ads and have a timeout before
        // you may go on the original link.
        Provider::PhxCoIn,
    ]
}

fn bambz_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"url\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        Some(string.to_owned().replace("\\", ""))
    } else {
        None
    }
}

fn bambz_request(url: &str, client: &Client) -> Option<Response> {
    client.post("https://bam.bz/api/short")
        .body(&format!("target={}", url))
        .header(ContentType::form_url_encoded())
        .send()
        .ok()
}


fn bngy_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<ShortenedUrl>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</ShortenedUrl>").next() {
        Some(string.to_owned())
    } else {
        None
    }
}

fn bngy_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://bn.gy/API.asmx/CreateUrl?real_url={}", url))
        .send()
        .ok()
}

fn fifocc_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"shortner\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        let mut short_url = string.to_owned();
        short_url = format!("http://fifo.cc/{}", short_url);
        Some(short_url)
    } else {
        None
    }
}

fn fifocc_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://fifo.cc/api/v2?url={}", url))
        .send()
        .ok()
}


fn hecsu_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<short>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</short>").next() {
        Some(string.to_owned())
    } else {
        None
    }
}

fn hecsu_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://hec.su/api?url={}&method=xml", url))
        .send()
        .ok()
}

fn isgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn isgd_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("https://is.gd/create.php?format=simple&url={}", url))
        .send()
        .ok()
}

fn nowlinks_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn nowlinks_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://nowlinks.net/api?url={}", url))
        .send()
        .ok()
}

fn phxcoin_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn phxcoin_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://phx.co.in/shrink.asp?url={}", url))
        .send()
        .ok()
}

fn psbeco_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let iter = string.split("<ShortUrl>").skip(1).next();
    if iter.is_none() {
        return None
    }
    if let Some(string) = iter.unwrap().split("</ShortUrl>").next() {
        Some(string.to_owned())
    } else {
        None
    }
}

fn psbeco_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://psbe.co/API.asmx/CreateUrl?real_url={}", url))
        .send()
        .ok()
}

fn rdd_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("\"rdd_url\"")
                      .nth(1).unwrap_or("")
                      .split(",").next().unwrap_or("")
                      .split("\"").nth(1);
    if let Some(string) = value {
        let mut short_url = string.to_owned();
        let _ = short_url.pop();
        Some(short_url)
    } else {
        None
    }
}

fn rdd_request(url: &str, client: &Client) -> Option<Response> {
    client.post("https://readability.com/api/shortener/v1/urls")
        .body(&format!("url={}", url))
        .send()
        .ok()
}

fn rlu_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn rlu_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://rlu.ru/index.sema?a=api&link={}", url))
        .send()
        .ok()
}

fn tinyurl_parse(res: &str) -> Option<String> {
    if res.is_empty() {
        return None
    }
    let string = res.to_owned();
    let value = string.split("data-clipboard-text=\"")
                      .nth(1).unwrap_or("")
                      .split("\">").next();
    if let Some(string) = value {
        Some(string.to_owned())
    } else {
        None
    }
}

fn tinyurl_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://tinyurl.com/create.php?url={}", url))
        .send()
        .ok()
}

fn vgd_parse(res: &str) -> Option<String> {
    Some(res.to_owned())
}

fn vgd_request(url: &str, client: &Client) -> Option<Response> {
    client.get(&format!("http://v.gd/create.php?format=simple&url={}", url))
        .send()
        .ok()
}


/// Parses the response from a successful request to a provider into the
/// URL-shortened string.
pub fn parse(res: &str, provider: Provider) -> Option<String> {
    match provider {
        Provider::BamBz => bambz_parse(res),
        Provider::BnGy => bngy_parse(res),
        Provider::FifoCc => fifocc_parse(res),
        Provider::HecSu => hecsu_parse(res),
        Provider::IsGd => isgd_parse(res),
        Provider::NowLinks => nowlinks_parse(res),
        Provider::PhxCoIn => phxcoin_parse(res),
        Provider::PsbeCo => psbeco_parse(res),
        Provider::Rdd => rdd_parse(res),
        Provider::Rlu => rlu_parse(res),
        Provider::TinyUrl => tinyurl_parse(res),
        Provider::VGd => vgd_parse(res),
    }
}

/// Performs a request to the short link provider.
/// Response to be parsed or `None` on a error.
pub fn request(url: &str, client: &Client, provider: Provider) -> Option<Response> {
    match provider {
        Provider::BamBz => bambz_request(url, client),
        Provider::BnGy => bngy_request(url, client),
        Provider::FifoCc => fifocc_request(url, client),
        Provider::HecSu => hecsu_request(url, client),
        Provider::IsGd => isgd_request(url, client),
        Provider::NowLinks => nowlinks_request(url, client),
        Provider::PhxCoIn => phxcoin_request(url, client),
        Provider::PsbeCo => psbeco_request(url, client),
        Provider::Rdd => rdd_request(url, client),
        Provider::Rlu => rlu_request(url, client),
        Provider::TinyUrl => tinyurl_request(url, client),
        Provider::VGd => vgd_request(url, client),
    }
}