use std::mem::MaybeUninit;
use std::sync::{Once, Mutex};
use crate::data::command::CommandBaog;

#[derive(Debug)]
pub struct BaogSettings{
    pub default_config_path: String,
    pub default_blogs_path: String,
}

impl Default for BaogSettings {
    fn default() -> Self {
        let username = dbg!(CommandBaog::get_home_name().unwrap());
        BaogSettings {
            default_config_path: if cfg!(windows) {
                format!("{}\\.baog", username) } else { format!("{}/.baog", username) },
            default_blogs_path: if cfg!(windows) {
                format!("{}\\BaogBlogs", username) } else { format!("{}/BaogBlogs", username) },

        }
    }
}

impl BaogSettings {

    pub fn get_app() -> &'static Mutex<BaogSettings> {
        static mut APP: MaybeUninit<Mutex<BaogSettings>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            APP.as_mut_ptr().write(Mutex::new(BaogSettings::default()))
        });

        unsafe {
            &*APP.as_ptr()
        }

    }
}

