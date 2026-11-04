use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::{env, fs};

const PREFIXES: [&str; 4] = ["ev_", "event_kind_", "notify_", "tray_"];

fn main() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("../messages");
    println!("cargo:rerun-if-changed={}", folder.display());

    let mut files: Vec<PathBuf> = fs::read_dir(&folder)
        .expect("the messages folder sits next to miner-core")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();

    let mut out = String::from("pub(crate) static TEXTS: &[(&str, &[(&str, &str)])] = &[\n");

    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let tag = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("a language file named by its tag");
        let raw = fs::read_to_string(&path).expect("a readable language file");
        let messages: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&raw).expect("a language file in JSON");

        writeln!(out, "    ({tag:?}, &[").unwrap();

        for (key, value) in &messages {
            let Some(text) = value.as_str() else { continue };

            if PREFIXES.iter().any(|prefix| key.starts_with(prefix)) {
                writeln!(out, "        ({key:?}, {text:?}),").unwrap();
            }
        }

        out.push_str("    ]),\n");
    }

    out.push_str("];\n");

    let target = Path::new(&env::var("OUT_DIR").expect("set by cargo")).join("texts.rs");
    fs::write(target, out).expect("OUT_DIR is writable");
}
