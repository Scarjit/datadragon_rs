
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub fn get(patch: &str, profile_icon_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/profile-icon/{}", patch, profile_icon_id))
}

