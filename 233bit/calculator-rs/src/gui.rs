extern crate native_windows_gui as nwg;
// use std::borrow::Borrow&Mut;
use std::fs;
use std::path::PathBuf;
use nwg::NativeUi;
use crate::dictread;
use walkdir::WalkDir;

#[derive(Default)]
pub struct Calculator {
    window: nwg::Window,
    layout: nwg::GridLayout,
    selctdictL: nwg::Label,
    selctitemL: nwg::Label,
    selctdict:nwg::ComboBox<&'static str>,
    selctitem:nwg::ComboBox<&'static str>,
    numberinputL:nwg::Label,
    numberinput:nwg::TextInput,
    searchbutton:nwg::Button,
    // compoundlist:nwg::RichTextBox,
    itemneedlist:nwg::RichTextBox,
    tree_view:nwg::TreeView,
}

impl Calculator {
    fn init_statr(&self){
        let showmessage = "使用方法:\n将txt配方表拖至该程序上即可,如:\n将示例文件expfile.txt拖拽至 合成材料计算器.exe上.\n示例文件在同级目录dict_dir下,若自己编写的合成表无法使用,请复制expfile.txt文件,在expfile.txt上进行修改\n\nnote:合成表必须使用英文符号".to_string();
        self.itemneedlist.set_text(&showmessage);
    }
    fn selctdictclick(&self,cobox:&nwg::ComboBox<&'static str>){
        while self.selctdict.len()>0 {
            self.selctdict.remove(0);
        }
        self.tree_view.clear();
        for entry in WalkDir::new(".\\dict_dir") {
            let ttmp = &entry.unwrap();
            if ttmp.path().to_str().unwrap().to_string().ends_with(".json"){
                let xe:&'static str = Box::leak(ttmp.path().to_str().unwrap().to_string().into_boxed_str());
                self.selctdict.push(xe);
            }

        }
    }
    fn flush_item_list(&self){
        while self.selctitem.len()>0 {
            self.selctitem.remove(0);
        }
        if let Some(xx) = self.selctdict.selection() {
        // println!("{}",self.selctdict.collection()[xx]);
        if let Ok(xez)= dictread::CompondList::json_to_self(self.selctdict.collection()[xx]){
            for idy in xez.item.keys(){
                self.selctitem.push(Box::leak(idy.clone().into_boxed_str()));
             } 
         }
        //  for idx in xez. 
         
        }
    }
    fn search_item_show_tree(&self,_bt:&nwg::Button){
        let tv = &self.tree_view;
        if let Some(xx1) = self.selctdict.selection(){
            if let Some(xx2) =self.selctitem.selection()  {
                if let Ok(xx3) = self.numberinput.text().parse::<u32>(){
                    // println!("XX3");
                    if let Ok(mut sseachitem) = dictread::CompondList::json_to_self(self.selctdict.collection()[xx1]){
                        tv.clear();
                        let getdata = sseachitem.search_itemv3(self.selctitem.collection()[xx2].to_string(), xx3);
                        // let mut disp_str = String::new();
                        // disp_str.push_str(string)
                        for idx in getdata.iter().enumerate(){
                            let item;
                            if idx.0==0{
                                item = tv.insert_item(&format!("[基础材料]"), None, nwg::TreeInsert::Root);
                            }else{
                                item = tv.insert_item(&format!("[{}级材料]",idx.0), None, nwg::TreeInsert::Root);

                                // disp_str.push_str(&format!("==========[----{}----]==========\n-> ",idx.0));
                            }
                            // let mut count1=0;
                            for idy in idx.1{
                                // count1+=1;
                                let d_m = idy.1/64;
                                let d_s = idy.1%64;
                                let mut sstr = String::new();
                                if d_m>0{
                                    if d_s>0{
                                        sstr = format!("{}*{}*64+{}",idy.0,d_m,d_s);
                                        // disp_str.push_str(&format!("[{}*{}*64+{}]",idy.0,d_m,d_s))
                                    }else{
                                        sstr = format!("{}*{}*64",idy.0,d_m);
                                        // disp_str.push_str(&format!("[{}*{}*64]",idy.0,d_m))
                                    }
                                }else{
                                    // disp_str.push_str(&format!("[{}*{}]",idy.0,idy.1))
                                    sstr = format!("{}*{}",idy.0,idy.1);
                                };
                                tv.insert_item(&sstr, Some(&item), nwg::TreeInsert::Last);
                                // if count1!=idx.1.len(){
                                //     disp_str.push_str(" , ")
                                // }
                            }
                            // println!("{}",&disp_str);
                            // disp_str.push_str("\n");
                        }
                        // self.itemneedlist.set_text(&disp_str);
                    }
                }
            }
        }{

        }
    }

    fn search_item_show(&self,_bt:&nwg::Button){
        if let Some(xx1) = self.selctdict.selection(){
            if let Some(xx2) =self.selctitem.selection()  {
                if let Ok(xx3) = self.numberinput.text().parse::<u32>(){
                    // println!("XX3");
                    if let Ok(mut sseachitem) = dictread::CompondList::json_to_self(self.selctdict.collection()[xx1]){
                        self.itemneedlist.clear();
                        let getdata = sseachitem.search_itemv3(self.selctitem.collection()[xx2].to_string(), xx3);
                        let mut disp_str = String::new();
                        // disp_str.push_str(string)
                        for idx in getdata.iter().enumerate(){
                            if idx.0==0{
                                disp_str.push_str(&format!("==========[基础材料]==========\n-> "));
                            }else{
                                disp_str.push_str(&format!("==========[----{}----]==========\n-> ",idx.0));
                            }
                            let mut count1=0;
                            for idy in idx.1{
                                count1+=1;
                                let d_m = idy.1/64;
                                let d_s = idy.1%64;
                                if d_m>0{
                                    if d_s>0{
                                        disp_str.push_str(&format!("[{}*{}*64+{}]",idy.0,d_m,d_s))
                                    }else{
                                        disp_str.push_str(&format!("[{}*{}*64]",idy.0,d_m))
                                    }
                                }else{
                                    disp_str.push_str(&format!("[{}*{}]",idy.0,idy.1))
                                };

                                if count1!=idx.1.len(){
                                    disp_str.push_str(" , ")
                                }
                            }
                            // println!("{}",&disp_str);
                            disp_str.push_str("\n");
                        }
                        self.itemneedlist.set_text(&disp_str);
                    }
                }
            }
        }{

        }
    }
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
    fn log_events(&self, evt: nwg::Event) {
        println!("{:?}", evt);
        if let Some(item) = self.tree_view.selected_item(){
            if let Some(selectedtext) =self.tree_view.item_text(&item){
                if selectedtext.contains("*"){
                    let prue_item:Vec<&str> = selectedtext.split("*").collect();
                    let prue_item = prue_item[0];
                    println!("{}",prue_item);
                    if let Some(xx1) = self.selctdict.selection(){
                        if let Ok(mut sseachitem) = dictread::CompondList::json_to_self(self.selctdict.collection()[xx1]){
                            let describtxt = sseachitem.read_descri_form_dict(prue_item);
                            self.itemneedlist.clear();
                            let describtxt = describtxt.replace(";", "\n");
                            self.itemneedlist.set_text(&describtxt);
                         }
                    } 
            // println!("{}",self.tree_view.item_text(&item).unwrap())
                }
        // println!("{}",self.tree_view.selected_item())
            }
        }
    
    }
}


mod calculator_ui {
    use native_windows_gui as nwg;
    use nwg::GridLayout;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct CalculatorUi {
        inner: Rc<Calculator>,
        default_handler: RefCell<Vec<nwg::EventHandler>>
    }

    impl nwg::NativeUi<CalculatorUi> for Calculator {
        fn build_ui(mut data: Calculator) -> Result<CalculatorUi, nwg::NwgError> {
            use nwg::Event as E;
            
            let mut ffont = nwg::Font::default();

            nwg::Font::builder().size(18).family("Microsoft YaHei").weight(500).build(&mut ffont);//.family("微软雅黑")
            // Controls
            nwg::Window::builder()
                // .size((300, 150))
                .size((700,400))
                .position((300, 300))
                .title("合成材料计算器")
                .build(&mut data.window)?;
          
            nwg::Label::builder()
                .text("选择字典")
                .font(Some(&ffont))
                .h_align(nwg::HTextAlign::Right)
                .v_align(nwg::VTextAlign::Top)
                .parent(&data.window)
                .build(&mut data.selctdictL)?;  
            nwg::Label::builder()
                .text("选择项目")
                .font(Some(&ffont))
                .h_align(nwg::HTextAlign::Right)
                .v_align(nwg::VTextAlign::Top)
                .parent(&data.window)
                .build(&mut data.selctitemL)?;   
            nwg::ComboBox::builder()
                .font(Some(&ffont))
                .parent(&data.window)
                .build(&mut data.selctdict)?;   
            nwg::ComboBox::builder()
                .font(Some(&ffont))
                .parent(&data.window)
                .build(&mut data.selctitem)?;   
            nwg::Label::builder()
                .text("输入数量")
                .font(Some(&ffont))
                .h_align(nwg::HTextAlign::Right)
                .parent(&data.window)
                .build(&mut data.numberinputL)?;   
            nwg::TextInput::builder()
                .text("")
                .font(Some(&ffont))
                .align(nwg::HTextAlign::Left)
                .parent(&data.window)
                .build(&mut data.numberinput)?;
            nwg::Button::builder()
                .text("查询")
                .font(Some(&ffont))
                .parent(&data.window)
                .build(&mut data.searchbutton)?;
            
            // nwg::RichTextBox::builder()
            //     .font(Some(&ffont))
            //     .readonly(true)
            //     .parent(&data.window)
            //     .build(&mut data.compoundlist)?;
            nwg::RichTextBox::builder()
                .font(Some(&ffont))
                .readonly(true)
                // .limit(2)
                .parent(&data.window)
                .build(&mut data.itemneedlist)?;
            nwg::TreeView::builder()
                .font(Some(&ffont))
                .parent(&data.window)
                .build(&mut data.tree_view)?;
            
       
            // Wrap-up
            let ui = CalculatorUi {
                inner: Rc::new(data),
                default_handler: Default::default()
            };

            // Events
            let window_handles = [&ui.window.handle];
            for handle in window_handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, _evt_data, handle| {
                    if let Some(evt_ui) = evt_ui.upgrade() {
                        match evt {
                            E::OnInit=>
                                if &handle ==&evt_ui.window{Calculator::init_statr(&evt_ui)}
                            E::OnComboBoxDropdown =>
                                if  &handle == &evt_ui.selctdict { Calculator::selctdictclick(&evt_ui, &evt_ui.selctdict)}
                            E::OnComboxBoxSelection=>
                                if &handle == &evt_ui.selctdict{Calculator::flush_item_list(&evt_ui)}
                            E::OnButtonClick=>
                                if &handle == &evt_ui.searchbutton {Calculator::search_item_show_tree(&evt_ui, &evt_ui.searchbutton);}
                            E::OnWindowClose => 
                                if &handle == &evt_ui.window {
                                    Calculator::exit(&evt_ui);
                                },
                            E::OnTreeItemSelectionChanged =>{
                                Calculator::log_events(&evt_ui, evt);
                            }
                            _ => {}
                        }
                    }
                };

                ui.default_handler.borrow_mut().push(
                    nwg::full_bind_event_handler(handle, handle_events)
                );
            }

            // Layouts
           

            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(1)
                // .margin([20,20,20,20])
                .min_size([150, 140])
                .child_item(nwg::GridLayoutItem::new(&ui.selctdictL, 0, 0, 1, 1))
                // .child(1, 0, &ui.selctdict)
                .child_item(nwg::GridLayoutItem::new(&ui.selctdict, 1, 0, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.selctitemL, 0, 1, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.selctitem, 1, 1, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.numberinputL, 2, 0, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.numberinput, 3, 0, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.searchbutton, 2, 1, 2, 1))
                // .child_item(nwg::GridLayoutItem::new(&ui.compoundlist, 0, 2, 2, 8))
                .child_item(nwg::GridLayoutItem::new(&ui.itemneedlist, 1, 2, 3, 7))
                
                .child_item(nwg::GridLayoutItem::new(&ui.tree_view, 0, 2, 1,7))
                // .child(1, 7, &ui.test_layout)
                // .child_item(nwg::GridLayoutItem::new(&ui.test_layout, 1, 7, 1,1))
                .build(&ui.layout)?;
            
            return Ok(ui);
        }
    }

    impl Drop for CalculatorUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for CalculatorUi {
        type Target = Calculator;

        fn deref(&self) -> &Calculator {
            &self.inner
        }
    }

}



pub fn main_run(){
    nwg::init().expect("Failed to init Native Windows GUI");
    // nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = Calculator::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events(); 
}