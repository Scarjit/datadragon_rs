
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub fn get_generic_square(patch: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/generic/square", patch))    
}

pub fn get_square(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/square", patch, champion_key))
}

pub fn get_data(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/data", patch, champion_key))
}

pub fn get_splash_art(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art", patch, champion_key))
}

pub fn get_splash_art_skin(patch: &str, champion_key: &str, skin_id: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/skin/{}", patch, champion_key, skin_id))
}

pub fn get_splash_art_centered(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/centered", patch, champion_key))
}

pub fn get_splash_art_centered_skin(patch: &str, champion_key: &str, skin_id: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/splash-art/centered/skin/{}", patch, champion_key, skin_id))
}

pub fn get_champ_select_sounds_ban(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/ban", patch, champion_key))
}

pub fn get_champ_select_sounds_choose(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/choose", patch, champion_key))
}

pub fn get_champ_select_sounds_sfx(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/champ-select/sounds/sfx", patch, champion_key))
}

pub fn get_tile(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/tile", patch, champion_key))
}

pub fn get_tile_skin(patch: &str, champion_key: &str, skin_id: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/tile/skin/{}", patch, champion_key, skin_id))
}

pub fn get_portrait(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/portrait", patch, champion_key))
}

pub fn get_portrait_skin(patch: &str, champion_key: &str, skin_id: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/portrait/skin/{}", patch, champion_key, skin_id))
}

pub fn get_ability_icon_passive(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/passive", patch, champion_key))
}

pub fn get_ability_icon_p(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/p", patch, champion_key))
}

pub fn get_ability_icon_q(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/q", patch, champion_key))
}

pub fn get_ability_icon_w(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/w", patch, champion_key))
}

pub fn get_ability_icon_e(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/e", patch, champion_key))
}

pub fn get_ability_icon_r(patch: &str, champion_key: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/champion/{}/ability-icon/r", patch, champion_key))
}

