use std::collections::HashMap;
use std::fmt::Display;
use reqwest::header::HeaderMap;
use reqwest::{Proxy, Url};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::ErrorType;
use crate::error::ErrorType::RequestError;


pub enum DownloadStat {
    MOD,
    FIle,
}

impl Display for DownloadStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadStat::MOD => write!(f, "mods"),
            DownloadStat::FIle => write!(f, "files"),
        }
    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub name_lower: String,
    pub forum_url: String,
    pub nexusmods_url: String,
    pub genre: String,
    pub file_count: u32,
    pub downloads: u32,
    pub domain_name: String,
    pub approved_date: u32,
    pub mods: u32,
    pub collections: u32,
}


pub struct NexusRequest {
    url: String,
    post: bool,
    cookie: String,
    proxy: Option<Url>,
}

impl NexusRequest {
    pub fn new<T: AsRef<str>>(cookie: T, proxy: Option<Url>) -> Self {
        Self {
            url: "".to_string(),
            post: false,
            cookie: cookie.as_ref().to_string(),
            proxy,
        }
    }

    pub fn set_url<T: AsRef<str>>(&mut self, url: T) {
        self.url = url.as_ref().to_string();
    }

    pub fn set_post(&mut self, post: bool) {
        self.post = post;
    }

    pub fn set_cookie<T: AsRef<str>>(&mut self, cookie: T) {
        self.cookie = cookie.as_ref().to_string();
    }

    pub fn set_proxy(&mut self, proxy: Option<Url>) {
        self.proxy = proxy;
    }
}

impl NexusRequest {

    /// 带请求头的N网定向包
    pub async fn request(&self) -> Result<Response, ErrorType> {
        let mut headers = HeaderMap::new();
        headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
        headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());

        if !self.cookie.is_empty() {
            headers.insert("Cookie", self.cookie.parse().unwrap());
        }

        // 创建请求
        let url = self.url.clone();
        let client = reqwest::Client::builder();

        // 代理
        let client = match self.proxy.clone() {
            Some(proxy) => {
                let p = Proxy::all(proxy).map_err(|v| { RequestError(v.to_string()) })?;
                client.proxy(p).build().map_err(|v| { RequestError(v.to_string()) })?
            }
            None => client.build().map_err(|v| { RequestError(v.to_string()) })?,
        };

        // 请求方式
        let cli = match self.post {
            true => client.post(url),
            false => client.get(url),
        };
        let res = cli.headers(headers).send().await.map_err(|v| { RequestError(v.to_string()) })?;
        Ok(res)
    }

    /// 获取mod下载统计
    pub async fn get_download_csv(&mut self, game_id: u32, _type: DownloadStat) -> Result<HashMap<String, (String, String, String)>, ErrorType>  {
        self.set_url(format!("https://staticstats.nexusmods.com/live_download_counts/{}/{}.csv", _type, game_id));
        self.set_post(false);

        let csv = self.request().await?.text().await.map_err(|v| { RequestError(v.to_string()) })?;
        let mut downloads_count = HashMap::new();

        for line in csv.lines() {
            let mut fields = line.split(',');
            let id = fields.next().unwrap_or_default().to_string();
            let download_all = fields.next().unwrap_or_default().to_string();
            let download_unique = fields.next().unwrap_or_default().to_string();
            match _type {
                DownloadStat::MOD => {
                    let look = fields.next().unwrap_or_default().to_string();
                    downloads_count.insert(id, (download_all, download_unique, look));
                }
                DownloadStat::FIle => {
                    downloads_count.insert(id, (download_all, download_unique, "".to_string()));
                }
            }
        }


        Ok(downloads_count)
    }

    /// 获取游戏列表
    pub async fn get_games_json(&mut self) -> Result<Vec<Game>, ErrorType>  {
        self.set_url("https://data.nexusmods.com/file/nexus-data/games.json");
        self.set_post(false);
        let v = self.request().await?;
        let res = v.json::<Vec<Game>>().await.map_err(|v| { RequestError(v.to_string()) })?;
        Ok(res)
    }

    /// 获取文件下载链接
    pub async fn get_file_link(&mut self, game_id: u32, file_id: u32) -> Result<String, ErrorType> {
        let url = format!("https://www.nexusmods.com/Core/Libs/Common/Managers/Downloads?GenerateDownloadUrl=&fid={}&game_id={}", file_id, game_id);
        self.set_url(url);
        self.set_post(true);
        let v = self.request().await?;
        let res = v.json::<Value>().await.map_err(|v| { RequestError(v.to_string()) })?;
        Ok(res["url"].to_string())
    }

}






