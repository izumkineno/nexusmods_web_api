
#[cfg(test)]
mod tests {
    use crate::parse::parse::get_mod_info;
    use crate::parse::selector::ModInfo;
    use crate::request::{DownloadStat, NexusRequest};
    use crate::test::{COOKIE, ID_GAME, PROXY};
    use crate::url_gen::mod_list::*;
    use crate::url_gen::NexusRequestUrl;
    use crate::url_gen::url_args::{SoftBy, TimePublish};

    #[tokio::test]
    async fn url_gen() {
        let mut url = ModListUrl::new(ID_GAME);
        println!("{}", url);
        url.set_sort_by(SoftBy::DatePublished);
        url.set_time(TimePublish::OneMonth);
        url.set_page(2);
        println!("{}", url);
        let res = url.request(COOKIE, Some(PROXY.parse().unwrap())).await.unwrap();
        dbg!(&res);
        // 解析
        let mut nr = NexusRequest::new(COOKIE, Some(PROXY.parse().unwrap()));
        let csv = nr.get_download_csv(ID_GAME, DownloadStat::MOD).await.unwrap();
        let res = get_mod_info(res, csv).unwrap();
        dbg!(&res);
        let res_struct = serde_json::from_value::<Vec<ModInfo>>(res).unwrap();
        dbg!(&res_struct);
    }
}