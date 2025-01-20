use std::{env, path::PathBuf, fs};
use serde_json::{Value, json};

fn main() {
    let mut args = env::args();
    let arg = args.nth(1).expect("missing first argument");
    let path = PathBuf::from(match arg.to_lowercase().as_str() {
        "bundler" => "pkg/bundler/package.json",
        "deno" => panic!("deno doesn't have a package.json"),
        "nodejs" => "pkg/nodejs/package.json",
        "web" => "pkg/web/package.json",
        got => panic!("got {got:?}, expected {:?}, {:?} or {:?}", "bundler", "nodejs", "web")
    });

    let content = fs::read_to_string(&path).unwrap();

    let mut package: Value = serde_json::from_str(&content).unwrap();
    let name = package.get_mut("name").unwrap();
    *name = json!("nu.js");

    let content = serde_json::to_string_pretty(&package).unwrap();
    fs::write(path, content).unwrap();
}
