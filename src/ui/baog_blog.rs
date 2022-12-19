use iced::{Element, Length};
use iced::widget::{Button, Column, Row, Text, TextInput};
use iced::{alignment, theme};
use crate::ui::{HomeMessage, HomePage};
use crate::ui::HomeMessage::ClickEvent;
use crate::ui::messages::{BlogPager, HomeEvent};
use crate::ui::messages::ClickEvents::HomeE;
use crate::ui::messages::HomeEvent::{MakeBlog, MakeLocal};


fn title_bar(back_page: BlogPager, title: String, forward_title: String, message: HomeMessage)
    -> Element<'static, HomeMessage> {

    let back = Text::new("<  返回")
        .horizontal_alignment(alignment::Horizontal::Center);
    let back = Button::new(back)
        .width(Length::Units(80))
        .style(theme::Button::Destructive)
        .on_press(HomeMessage::IntoBlog(back_page));
    let title = Text::new(format!("-{}-",title))
        .size(24)
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center);
    let forward = Text::new(forward_title)
        .horizontal_alignment(alignment::Horizontal::Center);
    let forward = Button::new(forward)
        .width(Length::Units(80))
        .style(theme::Button::Primary)
        .on_press(message);
    Row::new()
        .push(back)
        .push(title)
        .push(forward)
        .width(Length::Fill).height(Length::Shrink)
        .padding(12)
        .align_items(alignment::Alignment::Center)
        .into()
}


pub fn page_new_blog(data: &HomePage) -> Element<'static, HomeMessage> {
    let back_bar = title_bar(BlogPager::DefaultPage, "新建博客".into(),
                             "创建".into(), HomeMessage::ClickEvent(HomeE(MakeBlog)));

    let label_f = Text::new("博客框架：");
    let check_f = Text::new("Hexo")
        .width(Length::Units(420))
        .horizontal_alignment(alignment::Horizontal::Center);
    let row_blog_frame = Row::new().push(label_f).push(check_f)
        .spacing(20)
        .width(Length::Fill)
        .align_items(alignment::Alignment::End);

    let label_n = Text::new("博客名称： ");
    let input_n = TextInput::new("Blog name", &*data.page_new_input_name, HomeMessage::PageNewInputName)
        .width(Length::Units(420));
    let row_blog_name = Row::new().push(label_n).push(input_n)
        .spacing(20)
        .width(Length::Fill)
        .align_items(alignment::Alignment::End);

    let label_p = Text::new("存放地址： ");
    let input_p = TextInput::new("Path",&*data.page_new_input_path, HomeMessage::PageNewInputPath)
        .width(Length::Units(300));
    let but_p = Button::new("选择")
        .style(theme::Button::Positive).on_press(ClickEvent(HomeE(HomeEvent::ChooseFilePath)));
    let row_blog_path = Row::new().push(label_p).push(input_p).push(but_p)
        .spacing(20)
        .width(Length::Fill)
        .align_items(alignment::Alignment::End);

    let col = Column::new()
        .push(row_blog_frame)
        .push(row_blog_name)
        .push(row_blog_path)
        .padding(28)
        .spacing(10)
        .width(Length::Units(520)).height(Length::Shrink)
        .align_items(alignment::Alignment::Center);
    Column::new()
        .push(back_bar)
        .push(col)
        .width(Length::Fill).height(Length::Fill)
        .align_items(alignment::Alignment::Center)
        .spacing(14)
        .into()
}


pub fn page_import_local(data: &HomePage) -> Element<'static, HomeMessage> {
    let back_bar = title_bar(BlogPager::DefaultPage, "本地导入".into(),
                             "导入".into(), HomeMessage::ClickEvent(HomeE(MakeLocal)));


    let label_f = Text::new("博客框架：");
    let check_f = Text::new("Hexo")
        .width(Length::Units(300))
        .horizontal_alignment(alignment::Horizontal::Center);
    let row_blog_frame = Row::new().push(label_f).push(check_f)
        .spacing(20)
        .width(Length::Fill)
        .align_items(alignment::Alignment::End);

    let label_p = Text::new("导入地址： ");
    let input_p = TextInput::new("Path",&*data.page_new_input_path, HomeMessage::PageNewInputPath)
        .width(Length::Units(300));
    let but_p = Button::new("选择")
        .style(theme::Button::Positive).on_press(ClickEvent(HomeE(HomeEvent::ChooseFilePath)));
    let row_blog_path = Row::new().push(label_p).push(input_p).push(but_p)
        .spacing(20)
        .width(Length::Fill)
        .align_items(alignment::Alignment::End);

    let col = Column::new()
        .push(row_blog_frame)
        .push(row_blog_path)
        .padding(28)
        .spacing(10)
        .width(Length::Units(520)).height(Length::Shrink)
        .align_items(alignment::Alignment::Center);
    Column::new()
        .push(back_bar)
        .push(col)
        .width(Length::Fill).height(Length::Fill)
        .align_items(alignment::Alignment::Center)
        .spacing(14)
        .into()
}


pub fn page_import_repo(data: &HomePage) -> Element<'static, HomeMessage> {
    let back_bar = title_bar(BlogPager::DefaultPage, "仓库导入".into(),
                             "导入".into(), HomeMessage::None);

    let text = Text::new("暂不支持")
        .size(40);

    Column::new()
        .push(back_bar)
        .push(text)
        .width(Length::Fill).height(Length::Fill)
        .spacing(14)
        .align_items(alignment::Alignment::Center)
        .into()
}
