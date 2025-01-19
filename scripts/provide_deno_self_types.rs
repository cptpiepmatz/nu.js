use std::fs;

const JS_FILE_PATH: &'static str = "pkg/deno/nu_js.js";
const PROVIDER: &'static str  = r#"// @ts-self-types="./nu_js.d.ts""#;

fn main() {
    let content = fs::read_to_string(JS_FILE_PATH).unwrap();
    let content = format!("{PROVIDER}\n\n{content}");
    fs::write(JS_FILE_PATH, content).unwrap();
}
