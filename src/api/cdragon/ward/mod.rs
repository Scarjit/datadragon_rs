
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
pub fn get(patch: &str, ward_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/ward/{}", patch, ward_id))
}

pub fn get_shadow(patch: &str, ward_id: &str) -> Result<Vec<u8>, ()>{    
    cached_http_byte_request(format!("https://cdn.communitydragon.org/{}/ward/{}/shadow", patch, ward_id))
}

