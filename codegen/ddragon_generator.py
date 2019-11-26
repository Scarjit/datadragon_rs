import os
import re
import shutil

DDRAGON_ROOT = "..\\src\\api\\ddragon\\"
DDRAGON_WEB_ROOT = "https://ddragon.leagueoflegends.com/cdn/"
           


file_header = """
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/ddragon_generator.py
*/
use crate::tools::http::cached_http_byte_request;
"""

func_body_bytes = """
pub fn NAME() -> Result<Vec<u8>, ()>{
    cached_http_byte_request(URL.as_string())    
}
"""

func_body_json = """
pub fn NAME() -> Result<String, ()>{
    cached_http_json_request(URL.as_string())    
}
"""

def is_valid_folder(root):
    if "lolpatch_" in root:
        return False
    if "css" in root:
        return False
    if "js" in root:
        return False
    return True


def snakeify(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()  
     
     
def gen_file_root(root):
    if len(root) == 0:
        return DDRAGON_ROOT
    root = root.replace(".", "_").replace("-", "_")
    if root[0].isdigit():
        root = "d" + root
    
    if root[0] == '_':
        root = root[1:]
        
    return DDRAGON_ROOT + root

def generate_folders(foldername):
    for root, subdirs, files in os.walk(foldername):
        root = '\\'.join(root.split("\\")[1:])  
        
        if not is_valid_folder(root):
            continue        
            
        file_root = gen_file_root(root)
        
        try:
            os.mkdir(file_root)
        except FileExistsError:
            pass
 

def generate_func_name(file):
    return snakeify(file.split(".")[0])

def generate_func_url(file_root, file):
    url = "{}/{}".format(file_root.replace("\\", "/"), file)
    url = "\"" + DDRAGON_WEB_ROOT + '/'.join(url.split("/")[4:]) + "\""
    return url

def generate_file_sig(file_root, files):
    file_body = file_header
    for x in os.listdir(file_root):
        x = file_root + "\\" + x
        if os.path.isdir(x):
            folder_name = ''.join(x.split("\\")[-1:])
            file_body += "pub mod {};\n".format(folder_name)
    
    for file in files:
        if not "manifest" in file:
            if file.endswith(".json"):
                fbody = func_body_json
                func_name = generate_func_name(file)
                func_url = generate_func_url(file_root,file)
                fbody = fbody.replace("NAME", func_name)
                fbody = fbody.replace("URL", func_url)
                file_body += fbody
                
            elif file.endswith(".png") or file.endswith(".jpg") or file.endswith(".gif"):
                fbody = func_body_bytes
                func_name = generate_func_name(file)
                func_url = generate_func_url(file_root,file)
                fbody = fbody.replace("NAME", func_name)
                fbody = fbody.replace("URL", func_url)
                file_body += fbody
            elif file.endswith(".js"):
                pass
            else:
                print("Non parseable file: {}".format(file))
            
        
    return file_body
    
def generate_files(foldername):
    for root, subdirs, files in os.walk(foldername):
        root = '\\'.join(root.split("\\")[1:])
        
        if not is_valid_folder(root):
            continue
            
        file_root = gen_file_root(root)
        
        sig = generate_file_sig(file_root, files)
        with open(file_root + "\\mod.rs", "w") as f:
            f.write(sig)
            
    
    
if __name__ == "__main__":
    for x in os.listdir(os.getcwd()):
        if os.path.isdir(x):
            if "dragontail" in x:
                try:
                    shutil.rmtree(DDRAGON_ROOT)
                except FileNotFoundError:
                    pass
                generate_folders(x)
                generate_files(x)
                break