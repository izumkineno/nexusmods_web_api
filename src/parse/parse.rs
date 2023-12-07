use std::collections::HashMap;
use std::error::Error;
use chrono::DateTime;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use crate::parse::doc_to_json;
use crate::parse::selector::{FILES, IMG_LI, IMG_LI_ATTR, IMG_LI_MAIN1, IMG_LI_MAIN2, SELECT_MOD_LI, SELECT_MOD_LI_ATTR, SELECT_MOD_UR};


// 从mod_list中获取mod_info
pub fn get_mod_info(document: String, downloads_csv: HashMap<String, (String, String, String)>) -> Result<Value, Box<dyn Error>> {

    // 属性修正闭包
    let item_fix = |name: &str, v: &mut HashMap<&str, Value>| {
        match name {
            "last_update" => {
                // 修正时间格式
                let date = DateTime::parse_from_str(
                    &(v["last_update"].clone().as_str().unwrap().to_string() + " 00:00:00 +0000"),
                    "%d %b %Y %H:%M:%S%.3f %z",
                ).unwrap_or_default();
                let last_update = date.format("%Y-%m-%d").to_string();
                v.remove("last_update");
                v.insert("last_update", json!(last_update));
                // 添加下载量
                let download_count = match downloads_csv.get(v["id"].clone().as_str().unwrap()) {
                    None => "".to_string(),
                    Some(v) => v.clone().0,
                };
                v.insert("download_count", json!(download_count));
            },
            &_ => {}
        }
    };
    // 声明指针
    let item_fix: &dyn Fn(&str, &mut HashMap<&str, Value>) = &item_fix;

    let v = doc_to_json(&document, SELECT_MOD_UR, &SELECT_MOD_LI, &SELECT_MOD_LI_ATTR, item_fix)?;

    Ok(v)
}

pub fn get_mod_image(document: String) -> Result<Value, Box<dyn Error>> {
    let item_fix = |_name: &str, _v: &mut HashMap<&str, Value>| {};
    // 声明指针
    let item_fix: &dyn Fn(&str, &mut HashMap<&str, Value>) = &item_fix;

    let v1 = doc_to_json(&document, IMG_LI_MAIN1, &IMG_LI, &IMG_LI_ATTR, item_fix)?;
    let v2 = doc_to_json(&document, IMG_LI_MAIN2, &IMG_LI, &IMG_LI_ATTR, item_fix)?;
    Ok(json!({
        "images": v1,
        "images2": v2
    }))
}

pub fn get_mod_desc(document: String) -> Result<Value, Box<dyn Error>> {
    // todo 解析权限
    let fragment = Html::parse_document(&document);
    let sel_desc = Selector::parse("div.container.mod_description_container.condensed")?;
    let html = fragment.select(&sel_desc).next().unwrap().inner_html();
    Ok(json!({"description": html}))
}

pub fn get_mod_files(document: String) -> Result<Value, Box<dyn Error>> {
    let mut files = HashMap::new();
    for f in FILES {
        let fragment = Html::parse_document(&document);
        let sel_file = Selector::parse(f[1])?;
        let sel_file_description = Selector::parse(f[2])?;

        let file_main = fragment.select(&sel_file);
        let mut description = fragment.select(&sel_file_description);

        let file_info_vec = file_main.map(|v| {
            let description = match description.next() {
                None => "".to_string(),
                Some(v) => v.inner_html(),
            };
            let id = v.attr("data-id").unwrap_or_default();
            let name = v.attr("data-name").unwrap_or_default();
            let size = v.attr("data-size").unwrap_or_default();
            let version = v.attr("data-version").unwrap_or_default();
            let date = v.attr("data-date").unwrap_or_default();

            json!({
            "id": id,
            "name": name,
            "size": size,
            "version": version,
            "date": date,
            "description": description
        })
        }).collect::<Vec<_>>();
        files.insert(f[0], file_info_vec);
    }
    Ok(json!(files))
}

pub fn get_game_info(document: String) -> Result<Value, Box<dyn Error>> {
    let fragment = Html::parse_document(&document);
    let sel_file = Selector::parse("#games-list > div.game-section > ul > li")?;

    let games_id = fragment.select(&sel_file).map(|v| {
        v.attr("data-game-id").unwrap_or_default()
    }).collect::<Vec<_>>();
    
    Ok(json!(games_id))
}
