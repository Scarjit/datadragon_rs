use std::io::Read;
use std::io;
use cached::SizedCache;


cached_result!{
    BIN_CACHE: SizedCache<String, Vec<u8>> = SizedCache::with_size(512);
    fn cached_http_byte_request(url: String) -> Result<Vec<u8>, ()> = {
        match reqwest::get(&url) {
        Ok(v) => {
            let bytes_x = v.bytes();
            let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
            let mut bvec: Vec<u8> = vec![];
            for byte in bytes {
                bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
            }
            return Ok(bvec);
        },
        Err(_) => {
            Err(())
        },
    }
    }
}

cached_result!{
    STRING_CACHE: SizedCache<String, String> = SizedCache::with_size(512);
    fn cached_http_json_request(url: String) -> Result<String, ()> = {
        match reqwest::get(&url) {
            Ok(mut v) => {
                let text = v.text().expect("Couldn't convert answer to text");
                return Ok(text);
            },
            Err(_) => {
                Err(())
            },
        }
    }
}