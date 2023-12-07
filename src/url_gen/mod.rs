use std::error::Error;
use std::fmt::Display;
use reqwest::Url;
use crate::request::NexusRequest;

pub(crate) mod mod_list;
pub(crate) mod url_args;
pub(crate) mod mod_info;
pub(crate) mod game_list;


pub trait NexusRequestUrl {
    async fn request<T: AsRef<str>>(&self,cookie: T, proxy: Option<Url>) -> Result<String, Box<dyn Error>> where Self: Display {
        let mut req = NexusRequest::new(cookie, proxy);
        req.set_url(self.to_string());
        Ok(req.request().await?.text().await?)
    }
}
