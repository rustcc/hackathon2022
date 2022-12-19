use iced::{Element, Length, theme, alignment, Color};
use iced::widget::{Button, Column, Row, Text, Rule};
use iced_native::{Alignment};
use iced_native::widget::TextInput;
use crate::ui::baog_home::PageFilter::Home;
use crate::ui::{HomeMessage, HomePage};
use crate::ui::HomeMessage::{ClickEvent, FilterChanged};
use crate::ui::messages::{BlogPager, ClickEvents, HomeEvent};
use crate::ui::messages::ClickEvents::HomeE;
use crate::ui::messages::PluginsEvent::{HexoInstall, HexoUninstall};
use crate::ui::messages::HomeEvent::{ImportLocal, ImportRepo, NewProject};



pub fn page_control(filter_selected: PageFilter) -> Element<'static, HomeMessage> {

    let filter_buttons = |label, filter, filter_selected| {
        let label = Text::new(label)
            .vertical_alignment(alignment::Vertical::Center)
            .size(20);
        let button = Button::new(label)
            .width(Length::Fill)
            .height(Length::Units(40))
            .style( if filter == filter_selected {
                theme::Button::Positive
            } else {
                theme::Button::Text
            } );
        button.on_press(FilterChanged(filter)).padding(10)
    };

    Column::new()
        .push(filter_buttons("主页", Home, filter_selected))
        .push(filter_buttons("插件", PageFilter::Plugins, filter_selected))
        .push(filter_buttons("设置", PageFilter::Settings, filter_selected))
        .push(filter_buttons("关于", PageFilter::About, filter_selected))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

pub fn page_switch(data: &HomePage) -> Element<'static, HomeMessage> {


    // HomePage
    let home_page = move |data: &HomePage|-> Element<'static, HomeMessage> {

        let title = Text::new(" 轻松构建你的静态博客 ")
            .size(26)
            .width(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        let label_1 = Text::new("新建博客")
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(18);
        let button_create = Button::new(label_1)
            .style(theme::Button::Positive)
            .width(Length::Units(126))
            .on_press(ClickEvent(ClickEvents::HomeE(NewProject)))
            .on_press(HomeMessage::IntoBlog(BlogPager::NewBlog));
        let label_2 = Text::new("本地导入")
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(18);
        let button_import_local = Button::new(label_2)
            .style(theme::Button::Positive)
            .width(Length::Units(126))
            .on_press(ClickEvent(ClickEvents::HomeE(ImportLocal)))
            .on_press(HomeMessage::IntoBlog(BlogPager::ImportLocal));
        let label_3 = Text::new("仓库导入")
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(18);
        let button_import_vcs = Button::new(label_3)
            .style(theme::Button::Positive)
            .width(Length::Units(126))
            .on_press(ClickEvent(ClickEvents::HomeE(ImportRepo)))
            .on_press(HomeMessage::IntoBlog(BlogPager::ImportRepo));

        let row = Row::new()
            .push(button_create)
            .push(button_import_local)
            .push(button_import_vcs)
            .width(Length::Shrink)
            .height(Length::Units(30))
            .spacing(30)
            .align_items(Alignment::Fill);

        //-------------------------------------------------
        let divider = Rule::horizontal(1);
        //-------------------------------------------------


        let col = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::End)
            .padding(10);

        Column::new()
            .push(title)
            .push(row)
            .push(divider)
            .push(col)
            .width(Length::Fill).height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(32).into()
    };


    //PluginPage
    let plugins_page =  move |data: &HomePage| -> Element<'static, HomeMessage>{

        let title = Text::new(" 安装博客框架 ")
            .size(26)
            .width(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center);

        let hexo_t = Text::new("Hexo")
            .size(24)
            .width(Length::Shrink);
        let hexo_i = Text::new("需要 Node.js 环境")
            .size(16)
            .width(Length::Shrink);
        let hexo_i_node = Text::new(if data.is_npm_install {
            "npm已安装"} else{"npm未安装"})
            .size(16);
        let hexo_ti = Row::new()
            .push(hexo_t).push(hexo_i).push(hexo_i_node)
            .width(Length::Fill)
            .spacing(14)
            .align_items(Alignment::End);


        let hexo_isi =  data.is_hexo_install;
        let hexo_b_msg = if hexo_isi {
            Text::new("卸载").horizontal_alignment(alignment::Horizontal::Center)
        } else {
            Text::new("安装").horizontal_alignment(alignment::Horizontal::Center)
         };
        let hexo_b = Button::new(hexo_b_msg)
            .style(theme::Button::Primary)
            .width(Length::Units(90))
            .on_press(if hexo_isi { ClickEvent(ClickEvents::PluginsE(HexoUninstall))}
            else { ClickEvent(ClickEvents::PluginsE(HexoInstall)) });

        let hexo = Row::new()
            .push(hexo_ti)
            .push(hexo_b)
            .width(Length::Fill).height(Length::Shrink)
            .align_items(Alignment::End)
            .padding(10)
            .spacing(20);
        let hexo_d = Rule::horizontal(1);
        let hexo = Column::new()
            .push(hexo)
            .push(hexo_d);

        Column::new()
            .push(title)
            .push(hexo)
            .width(Length::Fill).height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(20).into()
    };


    //SettingPage
    let settings_page =  move |data: &HomePage| -> Element<'static, HomeMessage>{

        let title = Text::new(" 设置 ")
        .size(26)
        .width(Length::Fill)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center);

        let label_1 = Text::new("配置地址： ");
        let input_1 = TextInput::new("Path",&*data.page_default_config,
                                     HomeMessage::PageNewInputPath)
            .width(Length::Units(300));
        let but_1 = Button::new("选择")
            .style(theme::Button::Positive).on_press(ClickEvent(HomeE(HomeEvent::ChooseFilePath)));
        let row_blog_path = Row::new().push(label_1).push(input_1).push(but_1)
            .spacing(20)
            .width(Length::Shrink)
            .align_items(alignment::Alignment::End);

        Column::new()
            .push(title)
            .push(row_blog_path)
            .width(Length::Fill).height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(20).into()
    };


    //About Page
    let about_page =  move |data: &HomePage| -> Element<'static, HomeMessage> {

        let title = Text::new("BAnanablOG")
            .size(54)
            .style(theme::Text::Color(Color::from_rgb(0.2,0.6,0.3)));

        let info = Text::new("Make it easier to form a blog")
            .size(20)
            .style(theme::Text::Color(Color::from_rgb(0.2, 0.3, 0.4)));


        Column::new()
            .push(title)
            .push(info)
            .width(Length::Fill).height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(40)
            .spacing(20).into()
    };



    let showing_page = match data.filter {
        Home => {home_page(data)}
        PageFilter::Plugins => {plugins_page(data)}
        PageFilter::Settings => {settings_page(data)}
        PageFilter::About => {about_page(data)}
    };
    Column::new()
        .push(showing_page)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(16)
        .padding(10)
        .into()

}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PageFilter{
    Home,
    Plugins,
    Settings,
    About,
}

impl Default for PageFilter {
    fn default() -> Self {
        Home
    }
}

