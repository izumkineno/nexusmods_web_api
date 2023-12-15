use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::url_gen::NexusRequestUrl;
use super::url_args::{Order, SoftBy, TimePublish};

// todo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModListUrlAdvFilter {
    /// 默认false，高级过滤窗口，需要nav，
    open: bool,
    /// 文件大小，单位MB，最大19470
    range_size: [u16;2],
    /// 下载次数，最大18031102
    range_downloads: [u32;2],
    /// 点赞数（高级会员专属），最大358626
    range_endorsements: [u32;2],
    /// 游戏名包含内容
    search_filename: String,
    /// 详细信息包含内容
    search_description: String,
    /// 作者名包含内容
    search_author: String,
    /// 上传者名包含内容
    search_uploader: String
    // todo Refine with tags
    // todo Attributes
    // todo Refine by category
    // todo Language
}

impl Display for ModListUrlAdvFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rs = if self.range_size[1] == 0 {
            String::new()
        } else {
            format!(
                ",range_size:{}%3B{}",
                self.range_size[0],
                if self.range_size[1] > 19470 {
                    19470
                } else {
                    self.range_size[1]
                }
            )
        };

        let rd = if self.range_downloads[1] == 0 {
            String::new()
        } else {
            format!(
                ",range_downloads:{}%3B{}",
                self.range_downloads[0],
                if self.range_downloads[1] > 18031102 {
                    18031102
                } else {
                    self.range_downloads[1]
                }
            )
        };

        let re = if self.range_endorsements[1] == 0 {
            String::new()
        } else {
            format!(
                ",range_endorsements:{}%3B{}",
                self.range_endorsements[0],
                if self.range_endorsements[1] > 358626 {
                    358626
                } else {
                    self.range_endorsements[1]
                }
            )
        };

        let base = format!("open:{}", self.open);
        write!(f, "{}", base + &rs + &rd + &re)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModListUrl {
    /// 基础请求API
    base: &'static str,
    /// 默认false，仅请求mod
    nav: bool,
    /// ？？默认false
    home: bool,
    /// ？？默认true，高级过滤器
    advfilt: bool,
    /// ？？默认0，
    type_: u8,
    /// ？？默认0，用户id？？
    user_id: u32,
    /// 默认20，每页mod数，高级会员才能改 20 40 60 80
    page_size: u8,
    /// 默认false，是否显示游戏过滤
    show_game_filter: bool,
    /// 游戏id
    game_id: u32,
    /// 搜索过滤
    search_name: String,
    /// 默认false，是否包含 N S F W mod，需登录
    include_adult: bool,
    /// 默认1，当前页数
    page: u32,
    /// 默认发布时间，基于什么排序
    sort_by: SoftBy,
    /// 默认DESC，排序方式
    order: Order,
    /// 时间筛选
    time: TimePublish,
    /// 高级筛选器 todo
    filter: Option<ModListUrlAdvFilter>,
}

impl Display for ModListUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = format!(
            "{}nav:{},home:{},type:{},user_id:{},game_id:{},advfilt:{},include_adult:{},\
            show_game_filter:{},page_size:{},page:{}{}{}{}",
            self.base,
            self.nav,
            self.home,
            self.type_,
            self.user_id,
            self.game_id,
            self.advfilt,
            self.include_adult,
            self.show_game_filter,
            self.page_size,
            self.page,
            self.sort_by,
            self.order,
            self.time
        );

        let search_name = if self.search_name.is_empty() {
            String::new()
        } else {
            format!(",search%5Bfilename%5D:{}", self.search_name)
        };

        write!(f, "{}{}", base, search_name)
    }
}

impl ModListUrl {
    pub fn new(game_id: u32) -> ModListUrl {
        ModListUrl {
            base: "https://www.nexusmods.com/Core/Libs/Common/Widgets/ModList?RH_ModList=",
            nav: false,
            home: false,
            type_: 0,
            user_id: 0,
            game_id,
            advfilt: true,
            include_adult: false,
            search_name: "".to_string(),
            show_game_filter: false,
            page_size: 20,
            page: 1,
            sort_by: SoftBy::None,
            order: Order::None,
            time: TimePublish::None,
            filter: None,
        }
    }

    pub fn set_search_name(&mut self, search_name: impl AsRef<str>) {
        self.search_name = search_name.as_ref().to_string();
    }

    pub fn set_include_adult(&mut self, include_adult: bool) {
        self.include_adult = include_adult;
    }

    pub fn set_page(&mut self, page: u32) {
        self.page = page;
    }

    /// 默认20，每页mod数，高级会员才能改 20 40 60 80
    pub fn set_page_size(&mut self, page_size: u8) {
        self.page_size = page_size;
    }

    pub fn set_sort_by(&mut self, sort_by: SoftBy) {
        self.sort_by = sort_by;
    }

    pub fn set_order(&mut self, order: Order) {
        self.order = order;
    }

    pub fn set_time(&mut self, time_publish: TimePublish) {
        self.time = time_publish;
    }

    pub fn set_filter(&mut self, filter: ModListUrlAdvFilter) {
        self.filter = Some(filter);
    }

    pub fn clear_filter(&mut self) {
        self.filter = None;
    }

}

impl NexusRequestUrl for ModListUrl {}

