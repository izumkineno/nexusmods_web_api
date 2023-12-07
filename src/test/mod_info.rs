
#[cfg(test)]
mod tests {
    use crate::parse::parse::{get_mod_files, get_mod_image};
    use crate::test::{COOKIE, ID_GAME, ID_MOD, PROXY};
    use crate::url_gen::mod_info::*;
    use crate::url_gen::NexusRequestUrl;
    use crate::url_gen::url_args::ModInfoType;

    #[tokio::test]
    async fn url_gen() {
        let mut url = ModInfoUrl::new(ID_GAME, ID_MOD, ModInfoType::Description);
        println!("{}", url);
        url.set_type(ModInfoType::Files);
        println!("{}", url);
        url.set_type(ModInfoType::Images);
        let res = url.request(COOKIE, Some(PROXY.parse().unwrap())).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    async fn get_image_test() {
        let url = ModInfoUrl::new(ID_GAME, ID_MOD, ModInfoType::Images);
        let res = url.request(COOKIE, Some(PROXY.parse().unwrap())).await.unwrap();
        dbg!(&res);
        let res = get_mod_image(res).unwrap();
        dbg!(&res);
    }

    #[tokio::test]
    async fn get_files_test() {
        let url = ModInfoUrl::new(ID_GAME, ID_MOD, ModInfoType::Files);
        let res = url.request(COOKIE, Some(PROXY.parse().unwrap())).await.unwrap();
        dbg!(&res);
        let res = get_mod_files(res).unwrap();
        dbg!(&res);
    }
}