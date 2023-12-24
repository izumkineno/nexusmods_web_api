
#[cfg(test)]
mod tests {
    use crate::get_mod_info_page;
    use crate::parse::parse::get_mod_info;
    use crate::parse::selector::ModInfo;
    use crate::request::{DownloadStat, NexusRequest};
    use crate::test::{COOKIE, ID_GAME, PROXY};
    use crate::url_gen::mod_list::*;
    use crate::url_gen::NexusRequestUrl;

    #[tokio::test]
    async fn url_gen() {
        let mut url = ModListUrl::new(ID_GAME);
        url.set_nav(true);
        println!("{}", url);
        url.set_page(2);
        println!("{}", url);
        let res = url.request(COOKIE, PROXY).await.unwrap();
        // dbg!(&res);
        // 解析
        let mut nr = NexusRequest::new(COOKIE, PROXY);
        let csv = nr.get_download_csv(ID_GAME, DownloadStat::MOD).await.unwrap();
        let res1 = get_mod_info(&res, &csv).unwrap();
        // dbg!(&res1);
        let res_struct = serde_json::from_value::<Vec<ModInfo>>(res1).unwrap();
        // dbg!(&res_struct);
        let page = get_mod_info_page(&res);
        dbg!(&page);
    }
}
