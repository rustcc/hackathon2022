use serde::{Deserialize,Serialize};
use std::fs;
use std::collections::HashMap;
use std::io::{Write,Read, BufRead, BufReader};

#[derive(Debug,Serialize,Deserialize)]
pub struct CompondList{
    pub item: HashMap<String,HashMap<String,u32>>,
    pub describ: HashMap<String,String>,
}


impl CompondList{
    pub fn new()->Self{
        CompondList{item:HashMap::new(),describ:HashMap::new()}
    }
  
    pub fn to_string_local(&mut self,localp:String)->bool{
        if let  Ok(xx)= serde_json::to_string(self){
            if let Ok(mut file) =fs::File::create(localp)  {
                if let Ok(_)= file.write_all(xx.as_bytes()){
                    return true
                }
            }
        }
        false
    }
    pub fn json_to_self(txt:&str)->Result<CompondList, serde_json::Error>{
        let mut getstr = String::new();

        if let  Ok(mut file) = fs::OpenOptions::new().read(true).open(txt){
            file.read_to_string(&mut getstr).unwrap();
        }
        serde_json::from_str(&getstr)

    }
    pub fn read_descri_form_dict(&mut self,item:&str)->String{
        if let Some(val) = self.describ.get(item){
            val.to_string()
        }else{
            String::new()
        }
    }
    pub fn dict_to_self(txt:&str)->CompondList{
        // println!("1");
        let mut xxee=CompondList::new();
        if let  Ok( file) = fs::File::open(txt){
            let lines = BufReader::new(file).lines();
            for line in lines{
                if let Ok(lll)=line{
                    let xxeexx = lll.trim();//.replace(" ", "")
                    let getstr:Vec<&str>= xxeexx.split("=").collect();
                    if getstr.len()>=2{
                        let head = getstr[0].to_string().replace(" ", "");
                        let mut body = HashMap::new();
                        if let Some(_) = xxee.item.get(&head){
                            continue;
                        }
                        for childiitem in getstr[1].replace(" ", "").split("+"){
                            let c_c_2:Vec<&str> = childiitem.split("'").collect();
                            
                            if c_c_2.len()==2{
                                if let Ok(num_cc2) =c_c_2[0].parse::<u32>(){
                                    body.insert(c_c_2[1].to_string(), num_cc2);
                                }
                            }
                        }
                        if body.len()>0{
                            xxee.item.insert(head.clone(), body);
                            if getstr.len()==3{
                                xxee.describ.insert(head, format!("{}",getstr[2]));
                            }
                        }
                        
                    }
                }
            }
        }
        xxee
    }
   
    pub fn get_depth(& self,item_name:String)-> usize{
        if let Some(childitem)= self.item.get(&item_name.clone()){
            let mut lecount=0;
            // println!("11");
            for idx in childitem{
                let c_level =self.get_depth(idx.0.clone());
                if c_level>lecount{
                    lecount=c_level;
                }
            }
            lecount+1
        }else{
            0
        }
    }
    pub fn search_itemv3(&mut self,searchitem:String,compoud_number:u32)->Vec<HashMap<String, u32>>{
        let get_item_depth = self.get_depth(searchitem.clone());
        let mut return_dict:Vec<HashMap<String,u32>> =Vec::new();
        return_dict.resize(get_item_depth+1, HashMap::new());
        return_dict[get_item_depth].entry(searchitem.clone()).and_modify(|x| *x+=compoud_number).or_insert(compoud_number);
        let mut rc_list:HashMap<String,u32> = HashMap::new();
        rc_list.insert(searchitem,compoud_number);
        while !rc_list.is_empty() {
            let mut temp_rc_list:HashMap<String,u32> =HashMap::new(); 
            for idx in rc_list.clone(){
                
                if let Some(child_list) = self.item.get(&idx.0){
                for idy in child_list{
                    let dip_now = self.get_depth(idy.0.clone());

                    return_dict[dip_now].entry(idy.0.to_string()).and_modify(|x| *x+=idy.1*idx.1).or_insert(idy.1*idx.1);
                    temp_rc_list.entry(idy.0.clone()).and_modify(|x| *x+=idy.1.clone()*idx.1).or_insert(idy.1.clone()*idx.1);
                }
                } 
            }
        rc_list=temp_rc_list.clone();
        temp_rc_list.clear();
        }
    return_dict
    }
}