use axum::routing::MethodRouter;
use common::doc::ApiOperation;
pub mod model;
pub mod param;
pub mod prelude;

#[derive(Debug)]
pub struct AxumApiRoute<T: Send + Sync + Clone> {
    pub route: MethodRouter<T>,
    pub api: ApiOperation,
}

pub fn default_path(module_path: &str, crate_name: &str) -> String {
    let paths: Vec<&str> = module_path.split("::").collect();
    let mut result = String::default();
    let iter = paths.iter().skip(2);
    for item in iter {
        result = format!("{}/{}", result, item);
    }
    let case_rule = tools::case::RenameRule::SnakeCase;
    let mut result = format!("/{}{}", case_rule.apply_to_field(crate_name), result);
    if paths.len() > 1 {
        result = format!("/{}{}", paths[1], result);
    }
    result
}

pub fn parent_mod_name(module_path: &str) -> String {
    let paths: Vec<&str> = module_path.split("::").collect();
    let len = paths.len();
    if len > 2 {
        return paths[len - 2].to_string();
    }
    String::default()
}
