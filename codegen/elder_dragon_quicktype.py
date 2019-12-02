"""
    requires quicktype cli (https://www.npmjs.com/package/quicktype)
"""
import re
import os
import subprocess
from joblib import Parallel, delayed
import multiprocessing
import shutil

ELDER_ROOT = "..\\src\\api\\elderdragon\\serde\\"

def generate_mod_rs(top_path):
    files = []
    for x in os.listdir(top_path):
        path = top_path + "\\" + x
        if os.path.isdir(path):
            generate_mod_rs(path)
            files.append(x+".rs")
        elif os.path.isfile(path):
            files.append(x)    
    with open("{}\\mod.rs".format(top_path), "w") as f:
        for fx in files:
            if fx != "mod.rs":
                name = fx[:-3]
                
                if name == "serde":
                    f.write("""#[cfg(feature = "elder_serde")]
pub mod serde;\n""")
                elif name == "champion":
                    f.write("""#[cfg(feature = "elder_champ_gen")]
pub mod champion;\n""")
                else:
                    f.write("pub mod {};\n".format(name))

def snakeify(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()  
  
  
def execute(arg):
    print("Generating {}.rs".format(arg[6]))
    try:
        subprocess.run(arg)
    except Exception as e:
        with open("exception.log", "w") as f:
            f.write("{}\r\n\r\n{}".format(arg, e))
    print("Generated {}.rs".format(arg[6]))


def load_champion_jsons(_y):
    languages = []
    for x in os.listdir(_y):
        if os.path.isdir(_y + "\\" + x):
            languages.append(x)
    
    json_files = []
    for x in os.listdir(_y + "\\" + languages[0] + "\\champion"):
        j_path = _y + "\\" + languages[0] + "\\champion" + "\\" + x
        if os.path.isfile(j_path):
            json_files.append(x)
            
    file_dict = dict()
    
    for l in languages:
        lang_path = _y + "\\" + l
        for j in json_files:            
            json_path = _y + "\\" + l + "\\champion\\" + j
                        
            if not j in file_dict:
                file_dict[j] = []
                
            file_dict[j].append(json_path)
        
    args = []
    args_2 = []
    for k,v in file_dict.items():
        ki = k.replace(".", "_")
        
        for x in v:
            args_2.append(x)
    
        qt_args = [
            "{}\\npm\\quicktype.cmd".format(os.getenv('APPDATA')),
            "--lang",
            "rs",
            "--density",
            "dense",
            "--visibility",
            "public",
            "--top-level",
            ki,
            "--out",
            "{}champion\\{}.rs".format(ELDER_ROOT, snakeify(ki.replace("-", "_")))
        ]
        
        for vx in v:
            qt_args.append("--src")
            qt_args.append(vx)
        
        args.append(qt_args)
    
    qt_args = [
            "{}\\npm\\quicktype.cmd".format(os.getenv('APPDATA')),
            "--lang",
            "rs",
            "--density",
            "dense",
            "--visibility",
            "public",
            "--top-level",
            "combined",
            "--out",
            "{}champion\\_combined.rs".format(ELDER_ROOT),
            "--src",
            "tmp"
        ]
    try:
        shutil.rmtree("tmp")
    except FileNotFoundError:
        pass
    os.mkdir("tmp")
    
    for vx in args_2:
        shutil.copy(vx, "tmp")
    
    args.append(qt_args)
    
    num_cores = multiprocessing.cpu_count()
    results = Parallel(n_jobs=num_cores)(delayed(execute)(i) for i in args)
    

def load_basic_jsons(_y):
    languages = []
    for x in os.listdir(_y):
        if os.path.isdir(_y + "\\" + x):
            languages.append(x)
    
    json_files = []
    for x in os.listdir(_y + "\\" + languages[0]):
        j_path = _y + "\\" + languages[0] + "\\" + x
        if os.path.isfile(j_path):
            json_files.append(x)

    
    file_dict = dict()
    
    for l in languages:
        lang_path = _y + "\\" + l
        for j in json_files:            
            json_path = _y + "\\" + l + "\\" + j
            
            if not j in file_dict:
                file_dict[j] = []
                
            file_dict[j].append(json_path)
    
    args = []
    for k,v in file_dict.items():
        ki = k.replace(".", "_")
        qt_args = [
            "{}\\npm\\quicktype.cmd".format(os.getenv('APPDATA')),
            "--lang",
            "rs",
            "--density",
            "dense",
            "--visibility",
            "public",
            "--top-level",
            ki,
            "--out",
            "{}{}.rs".format(ELDER_ROOT, snakeify(ki.replace("-", "_")))
        ]
        
        for vx in v:
            qt_args.append("--src")
            qt_args.append(vx)
        
        args.append(qt_args)
    
    num_cores = multiprocessing.cpu_count()
    results = Parallel(n_jobs=num_cores)(delayed(execute)(i) for i in args)

serde_in_str = "extern crate serde_json;"
serde_str = """
use serde::{Serialize, Deserialize};
extern crate serde_json;
use self::serde_json::Error;

pub fn serialize(json: &str) -> Result<XTYPE,Error>{
    serde_json::from_str(json)
}
"""  
disclaimer = """
/*
    AUTO GENERATED FILE
    DO NOT EDIT
    codegen/elder_dragon_quicktype.py
*/
"""
  
def add_serde(froot):
    for root, dirs, files in os.walk(froot):
        for file in files:
            path = root + "\\" + file
            fin = open(path, "rt", encoding="utf-8")
            
            data = fin.read()
            data = re.sub(r"//.*", "", data)
            
            fname = re.findall(r'pub struct \w+', data)[0][11:]            
            
            sr = serde_str
            sr = sr.replace("XTYPE", fname)
            data = data.replace(serde_in_str, sr)
            
            data = disclaimer + data
            
            fin.close()

            fin = open(path, "wt", encoding="utf-8")
            fin.write(data)
            fin.close()


if __name__ == "__main__":
    try:
        shutil.rmtree(ELDER_ROOT)
    except FileNotFoundError:
        pass
    os.mkdir(ELDER_ROOT)
    os.mkdir(ELDER_ROOT + "champion\\")
    for x in os.listdir(os.getcwd()):
        if os.path.isdir(x):
            if "dragontail" in x:
                for y in os.listdir(os.getcwd() + "\\" + x):
                    if y[0].isdigit():
                        load_basic_jsons(os.getcwd() + "\\" + x + "\\" + y + "\\data")
                        load_champion_jsons(os.getcwd() + "\\" + x + "\\" + y + "\\data")
                        add_serde(ELDER_ROOT)
                        generate_mod_rs(ELDER_ROOT)