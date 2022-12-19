
#![windows_subsystem = "windows"]

mod dictread;
mod gui;
use std::env;
use std::path::Path;
fn main(){
    if !Path::new("./dict_dir").exists(){
        std::fs::create_dir("./dict_dir").unwrap();
        let text = String::from("钢锭=1'铁粉+1'碳+1'铁锭\n大马士革钢=1'铁锭+1'钢锭+1'铁粉+1'碳");
        std::fs::write("./dict_dir/expfile.txt",&mut format!("{}",text).as_bytes()).unwrap();
    }
    let args_all:Vec<String>=env::args().collect();
    if args_all.len()==2{
        let mut get = dictread::CompondList::dict_to_self(&args_all[1]);
        let workdictory = Path::new(&args_all[0]).parent();
        let filename = format!("{}\\dict_dir\\{}.json",workdictory.unwrap().to_str().unwrap(),Path::new(&args_all[1]).file_stem().unwrap().to_str().unwrap());
        get.to_string_local(filename);
    }
    gui::main_run();
}