use serde::Deserialize;
use std::{env, fs, path::PathBuf};

const REEXPORTS: &'static str = include_str!("./reexports.yml");

#[derive(Debug, Deserialize)]
struct Reexports {
    classes: Vec<String>,
}

mod esm {
    use indoc::indoc;

    #[fmt::skip]
    pub const REPLACER: &'static str = indoc!(r#"
        export function __nu_js__reexport__placeholder() {
            wasm.__nu_js__reexport__placeholder();
        }
    "#);

    pub fn replacement<'s>(with: impl IntoIterator<Item: AsRef<str>>) -> String {
        let exports = with
            .into_iter()
            .map(|item| format!("export {{ {} }};", item.as_ref()));
        itertools::intersperse(exports, "\n".to_string()).collect()
    }
}

mod commonjs {
    use indoc::indoc;

    #[fmt::skip]
    pub const REPLACER: &'static str = indoc!(r#"
        module.exports.__nu_js__reexport__placeholder = function() {
            wasm.__nu_js__reexport__placeholder();
        };
    "#);

    pub fn replacement(with: impl IntoIterator<Item: AsRef<str>>) -> String {
        let exports = with
            .into_iter()
            .map(|item| format!("module.exports.{item} = {item};", item = item.as_ref()));
        itertools::intersperse(exports, "\n".to_string()).collect()
    }
}

enum PkgKind {
    Bundler,
    Deno,
    Nodejs,
    Web,
}

impl PkgKind {
    fn path(&self) -> PathBuf {
        PathBuf::from(match self {
            PkgKind::Bundler => "pkg/bundler/nu_js_bg.js",
            PkgKind::Deno => "pkg/deno/nu_js.js",
            PkgKind::Nodejs => "pkg/nodejs/nu_js.js",
            PkgKind::Web => "pkg/web/nu_js.js",
        })
    }

    fn replacer(&self) -> &'static str {
        match self {
            PkgKind::Bundler | PkgKind::Deno | PkgKind::Web => esm::REPLACER,
            PkgKind::Nodejs => commonjs::REPLACER,
        }
    }

    fn replacement(&self, with: impl IntoIterator<Item: AsRef<str>>) -> String {
        match self {
            PkgKind::Bundler | PkgKind::Deno | PkgKind::Web => esm::replacement(with),
            PkgKind::Nodejs => commonjs::replacement(with),
        }
    }
}

fn main() {
    let mut args = env::args();
    let arg = args.nth(1).expect("missing first argument");
    let kind = match arg.trim().to_lowercase().as_str() {
        "bundler" => PkgKind::Bundler,
        "deno" => PkgKind::Deno,
        "nodejs" => PkgKind::Nodejs,
        "web" => PkgKind::Web,
        got => panic!(
            "got {got:?}, expected {:?}, {:?}, {:?} or {:?}",
            "bundler", "deno", "nodejs", "web"
        ),
    };

    let reexports: Reexports = serde_yml::from_str(REEXPORTS).unwrap();
    let path = kind.path();
    let replacer = kind.replacer();
    let replacement = kind.replacement(reexports.classes);

    let content = fs::read_to_string(&path).unwrap();
    if !content.contains(replacer) {
        panic!("replacer not found in {}", path.display())
    }
    let content = content.replace(replacer, &replacement);
    fs::write(path, content).unwrap();
}
