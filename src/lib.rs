#![crate_type = "lib"]
#![crate_name = "nexusmods_web_api"]


// mod test;
mod url_gen;
mod request;
mod parse;
mod error;

pub use url_gen::NexusRequestUrl;
pub use url_gen::url_args::{Order, SoftBy, TimePublish, GameSoftBy, ModInfoType};
pub use url_gen::game_list::GameListUrl;
pub use url_gen::mod_list::ModListUrl;
pub use url_gen::mod_info::ModInfoUrl;
pub use request::{NexusRequest, DownloadStat, Game};
pub use parse::selector::*;
pub use parse::parse::*;
