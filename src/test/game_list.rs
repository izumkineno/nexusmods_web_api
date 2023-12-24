
#[cfg(test)]
mod tests {
    use crate::test::{COOKIE, PROXY};
    use crate::url_gen::game_list::GameListUrl;
    use crate::url_gen::NexusRequestUrl;

    #[tokio::test]
    async fn url_gen() {
        let url = GameListUrl::new("sky");
        let res = url.request(COOKIE, PROXY).await.unwrap();
        dbg!(res);
    }
}
