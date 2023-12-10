use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub image: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub category: String,
    pub url_category: String,
    pub upload_date: String,
    pub last_update: String,
    pub author: String,
    pub uploader: String,
    pub url_uploader: String,
    pub size: String,
    pub endorse_count: String,
    pub download_count: String,
}


pub(crate) struct AttrName<'a> {
    /// CSS路径
    pub(crate) pos: usize,
    /// 元素在json中的名称
    pub(crate) name: &'a str,
    /// 元素在dom上的属性，为空则在dom内
    pub(crate) attr: &'a str,
    /// 文本在dom上的位置
    pub(crate) txt_pos: usize,
}

pub(crate) const SELECT_MOD_UR: &str = "#mod-list > ul.tiles > li";
pub(crate) const SELECT_MOD_LI: [&str; 11] = [
    "div.mod-tile-left",
    "div.mod-tile-left > a > figure > div > img",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > p.tile-name > a",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > p.desc",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > div > div.category > a",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > div > time",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > div > div.date",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > div > div.realauthor",
    "div.mod-tile-left > div.tile-desc.motm-tile > div.tile-content > div > div.author > a",
    "div.mod-tile-left > div.tile-data > ul > li.sizecount.inline-flex > span",
    "div.mod-tile-left > div.tile-data > ul > li.endorsecount.inline-flex > span",
];

pub(crate) const SELECT_MOD_LI_ATTR: [AttrName; 14] = [
    AttrName { pos: 0, name: "id", attr: "data-mod-id", txt_pos: 0 },
    AttrName { pos: 1, name: "image", attr: "src", txt_pos: 0 },
    AttrName { pos: 2, name: "title", attr: "", txt_pos: 0 },
    AttrName { pos: 2, name: "url", attr: "href", txt_pos: 0 },
    AttrName { pos: 3, name: "description", attr: "", txt_pos: 0 },
    AttrName { pos: 4, name: "category", attr: "", txt_pos: 0 },
    AttrName { pos: 4, name: "url_category", attr: "href", txt_pos: 0 },
    AttrName { pos: 5, name: "upload_date", attr: "datetime", txt_pos: 0 },
    AttrName { pos: 6, name: "last_update", attr: "", txt_pos: 1 },
    AttrName { pos: 7, name: "author", attr: "", txt_pos: 1 },
    AttrName { pos: 8, name: "uploader", attr: "", txt_pos: 0 },
    AttrName { pos: 8, name: "url_uploader", attr: "href", txt_pos: 0 },
    AttrName { pos: 9, name: "size", attr: "", txt_pos: 0 },
    AttrName { pos: 10, name: "endorse_count", attr: "", txt_pos: 0 },
];


pub(crate) const IMG_LI_MAIN1: &str = "#mod_images_list_1 > li > div > a";
pub(crate) const IMG_LI_MAIN2: &str = "#mod_images_list_2 > li > div > a";
pub(crate) const IMG_LI: [&str; 2] = [
    "figure > div > img",
    "div > p"
];
pub(crate) const IMG_LI_ATTR: [AttrName; 2] = [
    AttrName { pos: 0, name: "img", attr: "src", txt_pos: 0 },
    AttrName { pos: 1, name: "title", attr: "", txt_pos: 0 },
];


pub(crate) const FILES: [[&str; 3]; 4] = [
    [
        "main",
        "#file-container-main-files > div.accordionitems > dl > dt",
        "#file-container-main-files > div.accordionitems > dl > dd > div.tabbed-block.files-description > p"
    ],
    [
        "optional",
        "#file-container-optional-files > div.accordionitems > dl > dt",
        "#file-container-optional-files > div.accordionitems > dl > dd > div.tabbed-block.files-description > p"
    ],
    [
        "miscellaneous",
        "#file-container-miscellaneous-files > div.accordionitems > dl > dt",
        "#file-container-miscellaneous-files > div.accordionitems > dl > dd > div.tabbed-block.files-description > p"
    ],
    [
        "old",
        "#file-container-old-files > div.accordionitems > dl > dt",
        "#file-container-old-files > div.accordionitems > dl > dd > div.tabbed-block.files-description > p"
    ]
];



