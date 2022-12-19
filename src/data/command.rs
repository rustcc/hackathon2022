use std::{process::Command, error::Error};
use std::fs;
use std::path::Path;
use native_dialog::{MessageDialog, MessageType};
use crate::data::settings::BaogSettings;

#[allow(dead_code)]
pub struct CommandBaog;

impl CommandBaog {

    pub fn cmd_input(cmd: &str) -> Result<String, Box<dyn Error>> {
        let out = if !cfg!(windows) {
            Command::new(cmd)
            .output()?
        } else {
            Command::new("powershell").arg(cmd)
            .output()?
        };
        let info = String::from_utf8_lossy(&out.stdout);
        Ok(info.to_string())
    }


    pub fn cmd_with_args_root(cmd: &str, args: Vec<&str>) -> Result<String, Box<dyn Error>> {

        if !cfg!(windows) {
            use sudo::RunningAs;
            #[cfg(unix)]
            if let RunningAs::User = sudo::check() {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("run as su")
                    .set_text("请使用sudo运行本程序以进行安装或卸载！")
                    .show_alert()
                    .unwrap();
            }
        }

        let mut out = Command::new(cmd);
        for a in args {
            out.arg(a);
        }
        out.output()?;

        Ok("OK".to_string())
    }


    pub fn cmd_with_args(cmd: &str, args: Vec<&str>) -> Result<String, Box<dyn Error>> {
        let mut out = if !cfg!(windows) {
            Command::new(cmd)
        } else {
            Command::new("powershell")
        };
        if cfg!(windows) {
           out.arg(cmd);
        } else {};

        for a in args {
            out.arg(a);
        };
        out.output()?;

        Ok("OK".to_string())
    }

    pub fn get_home_name() -> Result<String, Box<dyn Error>> {
        use dirs;
        let out = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

        Ok(out)
    }

    pub fn make_config_dir() -> Result<String, Box<dyn Error>> {
        let config_dir = {BaogSettings::get_app().lock().unwrap().default_config_path.clone()};

        let config_file_dir = format!("{}/config.toml", config_dir.clone());
        if !Path::new(&config_dir).exists() {
            fs::create_dir(config_dir.clone()).expect("Error creating dir.");
        }
        if !Path::new(&config_file_dir).exists() {
            fs::write(config_file_dir.clone(),  "").expect("Error making file");
        }

        Ok("".to_string())
    }

}


