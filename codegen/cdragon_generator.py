import requests
import re

r = requests.get("https://cdn.communitydragon.org/endpoints").json()

fileheader = """
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/cdragon_generator.py
*/
use std::io::Read;
use std::io;
"""

func_body_1 = """
    let resp = reqwest::get(&format!("URL", ARG_1))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

"""
func_body_2 = """
    let resp = reqwest::get(&format!("URL", ARG_1, ARG_2))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

"""
func_body_3 = """
    let resp = reqwest::get(&format!("URL", ARG_1, ARG_2, ARG_3))?;
    let bytes_x = resp.bytes();
    let bytes = bytes_x.enumerate().collect::<Vec<(usize,Result<u8, io::Error>)>>();
    let mut bvec: Vec<u8> = vec![];
    for byte in bytes {
        bvec.push(byte.1.expect("Couldn't convert Bytes to Vec<u8>"))
    }
    Ok(bvec)
}

"""

function_sig_one = "pub fn NAME(ARG_1: &str) -> Result<Vec<u8>, reqwest::Error>{"
function_sig_two = "pub fn NAME(ARG_1: &str, ARG_2: &str) -> Result<Vec<u8>, reqwest::Error>{"
function_sig_three = "pub fn NAME(ARG_1: &str, ARG_2: &str, ARG_3: &str) -> Result<Vec<u8>, reqwest::Error>{"

def get_name(splits):   
    n = []
    for s in splits:
        if not s.startswith(":"):
            n.append(s)
    x =  "get_" + '_'.join(n)
    if len(x) == 4:
        return "get"
    else:
        return x

def snakeify(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
        
endpdict = dict()

for endpoint in r:
    splits = endpoint.split('/')
    folder = splits[4]
    
    args = []
    for s in splits:
        if s.startswith(":"):
            args.append(s.replace(":", ""))
    
    type = splits[4]
    name = get_name(splits[5:]).replace("-", "_")
    
    if type+name in endpdict:
        continue
    
    a_len = len(args)
    
    if a_len == 1:        
        body = function_sig_one
        body += func_body_1        
        for i,a in enumerate(args):
            endpoint = endpoint.replace(":" + a, "{}")
            body = body.replace("ARG_" + str(i+1), snakeify(args[i]))
        body = body.replace("NAME", name).replace("URL", endpoint)
        endpdict[type+name] = body
        
    elif a_len == 2:
        body = function_sig_two
        body += func_body_2        
        for i,a in enumerate(args):
            endpoint = endpoint.replace(":" + a, "{}")
            body = body.replace("ARG_" + str(i+1), snakeify(args[i]))            
        body = body.replace("NAME", name).replace("URL", endpoint)
        endpdict[type+name] = body
    elif a_len == 3:
        body = function_sig_three
        body += func_body_3        
        for i,a in enumerate(args):
            endpoint = endpoint.replace(":" + a, "{}")
            body = body.replace("ARG_" + str(i+1), snakeify(args[i]))            
        body = body.replace("NAME", name).replace("URL", endpoint)
        endpdict[type+name] = body
    else:
        print("Endpoint has more then 3 args")
        print(endpoint)
        
   
   
file_h = fileheader
file_c = fileheader
file_w = fileheader
file_p = fileheader

for k,v in endpdict.items():
    
    if "honor" in k:
        file_h += v        
    elif "champion" in k:
        file_c += v    
    elif "ward" in k:
        file_w += v    
    elif "profile" in k:
        file_p += v
    
    
with open("..\\src\\api\\cdragon\\honor\\mod.rs", "w") as f:
    f.write(file_h)
     
with open("..\\src\\api\\cdragon\\champion\\mod.rs", "w") as f:
    f.write(file_c)
    
with open("..\\src\\api\\cdragon\\ward\\mod.rs", "w") as f:
    f.write(file_w)
    
with open("..\\src\\api\\cdragon\\profile_icon\\mod.rs", "w") as f:
    f.write(file_p)
     
    
    
    
    