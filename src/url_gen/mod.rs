use std::fmt::Display;
use crate::error::ErrorType;
use crate::error::ErrorType::RequestError;
use crate::request::NexusRequest;

pub(crate) mod mod_list;
pub(crate) mod url_args;
pub(crate) mod mod_info;
pub(crate) mod game_list;


pub trait NexusRequestUrl {
    async fn request(&self, cookie: impl AsRef<str>, proxy: impl AsRef<str>) -> Result<String, ErrorType> where Self: Display {
        let mut req = NexusRequest::new(cookie, proxy);
        req.set_url(self.to_string());
        Ok(req.request().await?.text().await.map_err(|v| { RequestError(v.to_string()) })?)
    }
}
