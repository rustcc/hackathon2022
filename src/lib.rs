extern crate core;

use crate::data::command::CommandBaog;
use crate::data::settings::BaogSettings;

mod ui;
mod data;


pub fn run() {
    let _unused = BaogSettings::get_app().lock().unwrap();
    drop(_unused);

    CommandBaog::make_config_dir().unwrap();
    ui::create_home();
}