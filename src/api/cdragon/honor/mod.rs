
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub fn get_generic(patch: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/generic", patch))    
}

pub fn get(patch: &str, honor_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/{}", patch, honor_id))
}

pub fn get_locked(patch: &str, honor_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/{}/locked", patch, honor_id))
}

pub fn get_level(patch: &str, honor_id: &str, level: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/{}/level/{}", patch, honor_id, level))
}

pub fn get_emblem_generic(patch: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/emblem/generic", patch))    
}

pub fn get_emblem(patch: &str, honor_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/emblem/{}", patch, honor_id))
}

pub fn get_emblem_locked(patch: &str, honor_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/locked", patch, honor_id))
}

pub fn get_emblem_level(patch: &str, honor_id: &str, level: &str) -> Result<Vec<u8>, ()>{
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/honor/emblem/{}/level/{}", patch, honor_id, level))
}

