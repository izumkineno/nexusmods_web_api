use std::fmt::Display;
use crate::{GameSoftBy, Order};
use crate::url_gen::{NexusRequestUrl};

pub struct GameListUrl {
    name: String,
    order: Order,
    game_soft_by: GameSoftBy,
    page_size: u32,
    page: u32,
}

impl Display for GameListUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!(
            "https://www.nexusmods.com/Core/Libs/Common/Widgets/GamesList?RH_GamesList=f_name:{},order:{},sort_by:{},page_size:{},page:{}",
            self.name, self.order, self.game_soft_by, self.page_size, self.page);

        write!(f, "{}", url)
    }
}

impl NexusRequestUrl for GameListUrl {}

impl GameListUrl {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        GameListUrl {
            name: name.as_ref().to_string(),
            order: Order::DESC,
            game_soft_by: GameSoftBy::ModCount,
            page_size: 15,
            page: 1,
        }
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = order;
    }

    pub fn set_game_soft_by(&mut self, game_soft_by: GameSoftBy) {
        self.game_soft_by = game_soft_by;
    }

    /// 默认15，每页mod数，高级会员才能改
    pub fn set_page_size(&mut self, page_size: u32) {
        self.page_size = page_size;
    }

    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }
}


