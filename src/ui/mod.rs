mod baog_home;
mod messages;
mod events;
mod baog_blog;

use image::io::Reader as ImageReader;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::io::Cursor;
use std::{thread};

use iced::{Element, Length, Sandbox, Settings, Theme, alignment, window, Alignment};
use iced::widget::{Column, container, Row, Rule, Text};
use iced::window::{Icon, Position};
use crate::data::settings::BaogSettings;
use crate::ui::baog_blog::{page_import_local, page_import_repo, page_new_blog};
use crate::ui::baog_home::{page_control, page_switch, PageFilter};
use crate::ui::messages::{AboutEvent, BlogPager, ClickEvents, HomeEvent, PluginsEvent, SettingsEvent};
use crate::ui::events::{EventPlugins, PluginsCheck};

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub fn create_home() {

    // App id for application.
    let app_id = String::from("com.flatig.baog");
    // Font: 得意黑
    let _font_smiley_sans = include_bytes!(
        "../../res/font/SmileySans-Oblique.ttf");
    // Font: 站酷小薇Logo
    let _font_zhanku_xiaologo = include_bytes!(
        "../../res/font/ZhanKuXiaoLOGOTi-2.otf");
    // Font：Maple Mono
    let _font_maple_mono_regular = include_bytes!(
        "../../res/font/MapleMono-NF-Regular.ttf");


    let icon_window = Icon::from_rgba(
        ImageReader::new(Cursor::new(
            include_bytes!("../../res/img/ic_home.png")))
            .with_guessed_format().unwrap().decode().unwrap()
            .to_rgba8().to_vec()
        , 616, 616).unwrap();

    HomePage::run(Settings {
        id: Some(app_id),
        window: window::Settings {
            size: (800, 600),
            position: Position::Centered,
            min_size: Some((800, 600)),
            icon: Some(icon_window),
            ..window::Settings::default()
        },
        default_font: Some(_font_smiley_sans),
        default_text_size: 20,
        ..Settings::default()
    }).expect("Error while starting the BAOG");

}


pub struct HomePage {
    pager: BlogPager,
    filter: PageFilter,
    page_default_config: String,
    blog_list: Vec<BlogList>,

    is_npm_install: bool,
    is_hexo_install: bool,

    page_new_input_name: String,
    page_new_input_path: String,
}


pub struct BlogList {
    blog_name: String,
    blog_path: String,
}

impl BlogList {
    fn new(name: String, path: String) -> Self {
        Self {
            blog_name: name,
            blog_path: path,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    None,
    FilterChanged(PageFilter),
    ClickEvent(ClickEvents),
    IntoBlog(BlogPager),

    PageNewInputName(String),
    PageNewInputPath(String),
    SetNewConfigPath(String),
}


impl Sandbox for HomePage {
    type Message = HomeMessage;

    fn new() -> Self {
        HomePage {
            pager: BlogPager::default(),
            filter: PageFilter::default(),
            page_default_config: {
                let lock = BaogSettings::get_app().lock().unwrap();
                lock.default_config_path.clone()
            },
            blog_list: Vec::new(),

            is_npm_install: EventPlugins::check_install(PluginsCheck::Npm),
            is_hexo_install: EventPlugins::check_install(PluginsCheck::Hexo),

            page_new_input_name: String::from("blog"),
            page_new_input_path: {
                let lock = BaogSettings::get_app().lock().unwrap();
                lock.default_blogs_path.clone()
            },
        }
    }

    fn title(&self) -> String {
        String::from("Baog")
    }

    fn update(&mut self, message: Self::Message) {

        //messages
        match message {
            HomeMessage::FilterChanged(filter) => {
                self.filter = filter;
            },
            HomeMessage::ClickEvent(event) => {
                match event {
                    ClickEvents::None => {}
                    ClickEvents::HomeE(e) => { 
                        match e {
                            HomeEvent::None => {}
                            HomeEvent::NewProject => {}
                            HomeEvent::ImportLocal => {}
                            HomeEvent::ImportRepo => {}
                            HomeEvent::ChooseFilePath => {

                                if let Some(p) = FileDialog::new().show_open_single_dir().unwrap() {
                                    self.page_new_input_path = p.to_str().unwrap().to_string()
                                };
                            }
                            HomeEvent::MakeBlog => {
                                let path = self.page_new_input_path.clone();
                                let name = self.page_new_input_name.clone();
                                self.blog_list.push(BlogList::new(name.clone(), path.clone()));
                                thread::spawn(move || {
                                    MessageDialog::new()
                                        .set_type(MessageType::Info)
                                        .set_title("正在使用Hexo初始化博客，请稍等！")
                                        .set_text("请稍等！")
                                        .show_alert()
                                        .unwrap();
                                    if let Err(_)
                                        = EventPlugins::init_blog(PluginsCheck::Hexo,
                                                                  if cfg!(windows) {
                                                                      format!("{}\\{}", path, name)}
                                                                            else { format!("{}/{}", path, name)}) {
                                        MessageDialog::new()
                                            .set_type(MessageType::Error)
                                            .set_title("创建错误")
                                            .set_text("检查您的博客插件是否安装，检查填写的博客名称和路径是否正确！")
                                            .show_alert()
                                            .unwrap();
                                    };
                                });
                            }
                            HomeEvent::MakeLocal => {}
                            HomeEvent::MakeRepo => {}
                        }
                    }
                    ClickEvents::PluginsE(e) => {
                        match e { 
                            PluginsEvent::None => {}
                            PluginsEvent::HexoInstall => {

                                if dbg!(!EventPlugins::check_install(PluginsCheck::Npm)) {
                                    MessageDialog::new()
                                        .set_type(MessageType::Warning)
                                        .set_title("提示")
                                        .set_text("请先安装Node.js环境（含npm）")
                                        .show_alert()
                                        .unwrap();
                                    return;
                                }
                                self.is_npm_install = true;
                                MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("正在安装")
                                    .set_text("将在安装完成后提示，现在可以关闭此提示窗口。")
                                    .show_alert()
                                    .unwrap();
                                thread::spawn(|| {
                                    EventPlugins::install(PluginsCheck::Hexo);
                                    MessageDialog::new()
                                        .set_type(MessageType::Info)
                                        .set_title("安装完成")
                                        .set_text("Hexo 已经安装完成，现在可以在命令行工具中使用！")
                                        .show_alert()
                                        .unwrap();
                                });
                                if EventPlugins::check_install(PluginsCheck::Hexo) { self.is_hexo_install = true};
                            }
                            PluginsEvent::HexoUninstall => {

                                if !EventPlugins::check_install(PluginsCheck::Npm) {
                                    MessageDialog::new()
                                        .set_type(MessageType::Info)
                                        .set_title("卸载完成")
                                        .set_text("Hexo早已连带node.js消失不见！")
                                        .show_alert()
                                        .unwrap();
                                    self.is_hexo_install = false; self.is_hexo_install =false;
                                    return;
                                }
                                MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("正在卸载")
                                    .set_text("将在卸载完成后提示，现在可以关闭此提示窗口。")
                                    .show_alert()
                                    .unwrap();
                                thread::spawn(|| {
                                    EventPlugins::uninstall(PluginsCheck::Hexo);
                                    MessageDialog::new()
                                        .set_type(MessageType::Info)
                                        .set_title("卸载完成")
                                        .set_text("Hexo 已经完成卸载，请按需手动卸载Node.js。")
                                        .show_alert()
                                        .unwrap();
                                });
                                if EventPlugins::check_install(PluginsCheck::Hexo) { self.is_hexo_install = false };
                            }
                        }
                    }
                    ClickEvents::SettingsE(e) => {
                        match e { SettingsEvent::None => {} }
                    }
                    ClickEvents::AboutE(e) => {
                        match e { AboutEvent::None => {} }
                    }
                }
            }

            HomeMessage::IntoBlog(pager) => {
                match pager {
                    BlogPager::DefaultPage => {
                        self.pager = BlogPager::DefaultPage
                    }
                    BlogPager::NewBlog => {
                        self.pager = BlogPager::NewBlog
                    }
                    BlogPager::ImportLocal => {
                        self.pager = BlogPager::ImportLocal
                    }
                    BlogPager::ImportRepo => {
                        self.pager = BlogPager::ImportRepo
                    }
                    BlogPager::Blog => {
                        self.pager = BlogPager::Blog
                    }
                    BlogPager::None => {}
                }
            }
            HomeMessage::PageNewInputName(str) => {
                self.page_new_input_name = str
            }
            HomeMessage::PageNewInputPath(str) => {
                self.page_new_input_path = str
            }
            HomeMessage::SetNewConfigPath(str) => {
                self.page_default_config = str
            }
            HomeMessage::None => {}
        }

    }

    fn view(&self) -> Element<'_, Self::Message> {

        let text_baog = Text::new("B A O G")
            .height(Length::Shrink)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(32);

        let divider = Rule::horizontal(1);

        let column_left = Column::new()
            .push(text_baog)
            .push(divider)
            .push(page_control(self.filter))
            .width(Length::Units(240))
            .height(Length::Fill)
            .spacing(32)
            .padding(10);

        //-------------------------------------------------------
        let vertical_divider = Rule::vertical(6);
        //-------------------------------------------------------

        let column_right = Column::new()
            .push(page_switch(self))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(18);

        let main_row = match self.pager {
            BlogPager::DefaultPage => {
                Row::new()
                    .push(column_left)
                    .push(vertical_divider)
                    .push(column_right)
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            BlogPager::NewBlog => {
                Row::new()
                    .push(page_new_blog(self))
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            BlogPager::ImportLocal => {
                Row::new()
                    .push(page_import_local(self))
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            BlogPager::ImportRepo => {
                Row::new()
                    .push(page_import_repo(self))
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            BlogPager::Blog => {
                Row::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            BlogPager::None => {
                Row::new()
            }
        };

        container(main_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(14)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }

}
