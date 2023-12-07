
#[cfg(test)]
mod tests {
    use crate::request::{DownloadStat, NexusRequest};
    use crate::test::{COOKIE, ID_FILE, ID_GAME, PROXY};

    #[tokio::test]
    async fn request_test() {
        let mut req = NexusRequest::new(COOKIE, Some(PROXY.parse().unwrap()));
        let j = req.get_games_json().await.unwrap();
        dbg!(j);
    }

    #[tokio::test]
    async fn request2_test() {
        let mut req = NexusRequest::new(COOKIE, Some(PROXY.parse().unwrap()));
        let j = req.get_download_csv(ID_GAME, DownloadStat::FIle).await.unwrap();
        dbg!(j);
    }

    #[tokio::test]
    async fn request2_1_test() {
        let mut req = NexusRequest::new(COOKIE, Some(PROXY.parse().unwrap()));
        let j = req.get_download_csv(ID_GAME, DownloadStat::MOD).await.unwrap();
        dbg!(j);
    }

    #[tokio::test]
    async fn request3_test() {
        let mut req = NexusRequest::new(COOKIE, Some(PROXY.parse().unwrap()));
        let j = req.get_file_link(ID_GAME, ID_FILE).await.unwrap();
        println!("{}", j);
    }
}