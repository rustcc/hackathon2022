
#![allow(dead_code)]

/*
    Baog Home Messages
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClickEvents {
    None,
    HomeE(HomeEvent),
    PluginsE(PluginsEvent),
    SettingsE(SettingsEvent),
    AboutE(AboutEvent),
}
impl Default for ClickEvents {
    fn default() -> Self {
        ClickEvents::None
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HomeEvent {
    None,
    NewProject,
    ImportLocal,
    ImportRepo,
    ChooseFilePath,
    MakeBlog,
    MakeLocal,
    MakeRepo,
}
impl Default for HomeEvent {
    fn default() -> Self {
        HomeEvent::None
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PluginsEvent {
    None,
    HexoInstall,
    HexoUninstall,
}
impl Default for PluginsEvent {
    fn default() -> Self {
        PluginsEvent::None
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SettingsEvent {
    None,
}
impl Default for SettingsEvent {
    fn default() -> Self {
        SettingsEvent::None
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AboutEvent {
    None,
}
impl Default for AboutEvent {
    fn default() -> Self {
        AboutEvent::None
    }
}


/*
    Baog Blog Messages
 */

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlogPager {
    None,
    DefaultPage,
    NewBlog,
    ImportLocal,
    ImportRepo,
    Blog,
}

impl Default for BlogPager {
    fn default() -> Self {
        BlogPager::DefaultPage
    }
}