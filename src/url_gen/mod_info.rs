use std::fmt::Display;
use crate::url_gen::NexusRequestUrl;
use crate::url_gen::url_args::ModInfoType;

/// 获取mod详细信息
#[derive(Debug)]
pub struct ModInfoUrl {
    game_id: u32,
    mod_id: u32,
    _type: ModInfoType,
}
impl NexusRequestUrl for ModInfoUrl {}
impl Display for ModInfoUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://www.nexusmods.com/Core/Libs/Common/Widgets/{}?id={}&game_id={}", self._type, self.mod_id, self.game_id);
        write!(f, "{}", url)
    }
}

impl ModInfoUrl {
    pub fn new(game_id: u32, mod_id: u32, _type: ModInfoType) -> Self {
        ModInfoUrl {
            game_id,
            mod_id,
            _type,
        }
    }

    pub fn set_game_id(&mut self, game_id: u32) {
        self.game_id = game_id
    }

    pub fn set_mod_id(&mut self, mod_id: u32)  {
        self.mod_id = mod_id
    }

    pub fn set_type(&mut self, _type: ModInfoType) {
        self._type = _type
    }
}
