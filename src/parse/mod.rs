use std::collections::HashMap;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use crate::error::ErrorType;
use crate::parse::selector::AttrName;

pub(crate) mod parse;
pub(crate) mod selector;

pub fn selector_parse_error_fix(sel: &str) -> Result<Selector, ErrorType> {
    Ok(Selector::parse(sel).map_err(|v| { ErrorType::ParseError(v.to_string()) })?)
}


pub fn doc_to_json(
    document: &String,
    main_select: &'static str,
    selects: &[&str],
    selects_attrs: &[AttrName],
    item_fix: &dyn Fn(&str, &mut HashMap<&str, Value>)
) -> Result<Value, ErrorType> {
    let fragment = Html::parse_fragment(document);
    let sel_ur = selector_parse_error_fix(main_select)?;
    let ur = fragment.select(&sel_ur);
    let selects_vec: Vec<_> = selects.iter().map(|v| Selector::parse(v).unwrap()).collect();
    let mut single_vec = Vec::new();
    // 搜寻main_select下的selects
    for u in ur {
        let mut single = HashMap::new();

        for v in selects_attrs {
            let mut sel = u.select(&selects_vec[v.pos]);
            let t = match v.attr {
                "" => {
                    let v1 = sel.next().unwrap().text().collect::<Vec<_>>();
                    match v1.len() {
                        0 => "",
                        _ => v1[v.txt_pos].trim()
                    }
                },
                _ => sel.next().unwrap().attr(v.attr).unwrap_or_default()
            };

            single.insert(v.name, json!(t));
            item_fix(v.name, &mut single);
        }

        single_vec.push(single);
    }
    Ok(json!(single_vec))
}

