use std::sync::Once;
use regex::Regex;
use crate::tools::version_cmp::compare;

static START: Once = Once::new();
static mut VERSIONS: Vec<String> = Vec::new();
pub fn get() -> &'static Vec<String>{
    START.call_once(|| {
        let url = "http://raw.communitydragon.org/";
        let site_content = reqwest::get(url).expect("CDragon is not reachable").text().expect("Couldn't convert to string");

        let table = table_extract::Table::find_first(&site_content).unwrap();

        let mut vs: Vec<String> = vec![];
        let ree = Regex::new(r"\d.\d+").expect("Couldn't compile regex");
        for row in &table{
            let cell =  row.get(&"<a href=\"?C=N&amp;O=A\">File Name</a>&nbsp;<a href=\"?C=N&amp;O=D\">&nbsp;↓&nbsp;</a>").unwrap();
            match ree.find(cell) {
                None => {},
                Some(v) => {
                    vs.push(v.as_str().to_string())
                },
            }
        }

        vs.sort_by(|a, b| compare(a, b));


        unsafe {
            VERSIONS = vs;
        }
    });

    return unsafe { &VERSIONS };
}

pub fn is_latest(version: &str) -> bool{
    version == get().last().expect("No versions parsed")
}

pub fn latest() -> &'static str{
    get().last().expect("No versions parsed")
}