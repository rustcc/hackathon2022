use std::error::Error;
use native_dialog::{MessageDialog, MessageType};
use crate::data::command::CommandBaog;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PluginsCheck {
    Npm,
    Hexo,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EventPlugins;

impl EventPlugins {

    pub fn check_install(plugin: PluginsCheck) -> bool {
        let mut check: bool = false;

        match plugin {
            PluginsCheck::Hexo => {
                match dbg!(CommandBaog::cmd_input("hexo")) {
                    Ok(_) => check = true,
                    Err(_) => {},
                }
            }
            PluginsCheck::Npm => {
                match dbg!(CommandBaog::cmd_input("npm")) {
                    Ok(_) => check = true,
                    Err(_) => {},
                }
            }
        }
        check
    }


    pub fn install(plugin: PluginsCheck) -> bool {
        let mut result: bool = false;

        match plugin {
            PluginsCheck::Npm => { panic!("Install!") }
            PluginsCheck::Hexo => {
                CommandBaog::cmd_with_args_root("npm", vec!["install", "hexo-cli", "-g"])
                    .expect("Error using 'npm install'");
                result = true
            }
        }
        result
    }

    pub fn uninstall(plugin: PluginsCheck) -> bool {
        match plugin {
            PluginsCheck::Npm => { panic!("Uninstall!")}
            PluginsCheck::Hexo => {
                CommandBaog::cmd_with_args_root("npm", vec!["uninstall", "hexo-cli", "-g"])
                    .expect("Error using 'npm install'");
            }
        }
        true
    }

    pub fn init_blog(plugin: PluginsCheck, path: String) -> Result<(), Box<dyn Error>> {

        match plugin {
            PluginsCheck::Npm => { panic!("Uninstall!")}
            PluginsCheck::Hexo => {
                CommandBaog::cmd_with_args("hexo", vec!["init", path.as_str()])?;
                CommandBaog::cmd_with_args("npm", vec!["install"])?;
                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title("博客初始化成功")
                    .set_text("现在返回首页开始管理您的博客")
                    .show_alert()
                    .unwrap();
            }
        }
        Ok(())
    }

}