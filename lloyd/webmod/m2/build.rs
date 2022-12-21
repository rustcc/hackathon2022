use std::fs::{self, ReadDir};
use std::path::Path;

const DIR_PATH: &str = "api";

fn main() {
    let root_dir = format!("./src/{}/", DIR_PATH);
    let root_path = Path::new(root_dir.as_str());
    if !root_path.exists() || root_path.is_file() {
        return;
    }
    let read_dir = std::fs::read_dir(root_dir.as_str()).unwrap();
    let mut paths = vec![];
    let crate_name_path = format!("{}/mod.rs", root_dir);
    let crate_name = get_doc_name(&crate_name_path);
    handler_path(&mut paths, read_dir, DIR_PATH, crate_name);
    let content = gen_code(paths);
    let dest_path = Path::new("./src/gen_router.rs");
    fs::write(dest_path, content).unwrap();
    println!("cargo:rerun-if-changed={}", root_dir);
}

#[derive(Debug)]
struct RouterPath {
    pub path: String,
    pub mod_path: Option<String>,
}

fn handler_path(
    vec: &mut Vec<RouterPath>,
    read_dir: ReadDir,
    prefix: &str,
    mod_path: Option<String>,
) {
    for item in read_dir {
        let item = item.unwrap();
        let file_name = item.file_name();
        let file_name: String = file_name.to_str().unwrap().into();
        if file_name.eq("mod.rs") {
            continue;
        }
        if file_name.eq(".") || file_name.eq("..") {
            continue;
        }
        let meta = item.metadata().unwrap();
        let router_path = format!("{}::{}", prefix, file_name.split(".rs").next().unwrap());
        if meta.is_dir() {
            let item_path = item.path();
            let doc_path = item_path.join("mod.rs");
            let mod_name = get_doc_name(doc_path).unwrap_or(file_name);
            let sub_dir = std::fs::read_dir(&item_path).unwrap();
            let mod_path = match &mod_path {
                Some(mod_path) => {
                    format!("{}::{}", mod_path, mod_name)
                }
                None => mod_name,
            };
            handler_path(vec, sub_dir, router_path.as_str(), Some(mod_path));
            continue;
        }
        if !file_name.ends_with(".rs") {
            continue;
        }
        vec.push(RouterPath {
            path: router_path,
            mod_path: mod_path.clone(),
        });
    }
}

fn get_doc_name<P: AsRef<Path>>(path: P) -> Option<String> {
    let content = fs::read_to_string(path).unwrap();
    for item in content.lines() {
        let item = item.trim();
        if item.starts_with("//") && item.contains("doc:") {
            let result = item
                .split_once("doc:")
                .map(|x| x.1)
                .unwrap_or_default()
                .trim()
                .into();
            return Some(result);
        }
    }
    None
}

fn gen_code(paths: Vec<RouterPath>) -> String {
    let mut result_use = r#"
// 
// 此文件由build.rs文件自动生成， 请勿修改
// 
    
use axum::Router;
use crate::state::AppState;
use common::doc::ApiDoc;
use webase::router::merge;"#
        .to_string();
    result_use = format!("{}\r\nuse crate::{};", result_use, DIR_PATH);
    let mut result = r#"

pub fn router(doc: &mut ApiDoc) -> Router<AppState> {
    let mut router = Router::new();
    "#
    .to_string();
    result = format!("{}{}", result_use, result);
    for item in paths {
        let mod_path = match item.mod_path {
            Some(mod_path) => format!("Some(\"{}\")", mod_path),
            None => String::from("None"),
        };
        let router = format!(
            "    router = merge(
                doc,
                {}::handle_route(),
                router,
                {},
            );",
            item.path, mod_path,
        );
        result = format!("{}\r\n{}", result, router);
    }
    result = format!("{}\r\n    router\r\n}}", result);
    result
}
