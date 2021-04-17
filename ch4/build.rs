use std::{env, path::Path, process::Command};
use tera::{Context, Tera};
use vergen::{vergen, Config};

fn main() {
    built::write_built_file().expect("built failed");
    vergen(Config::default()).expect("vergen failed");
    gen_ch4_version();
    write_main();
}

fn gen_ch4_version() {
    let mut ch4_version = env::var("CARGO_PKG_VERSION").unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir);
    match built::util::get_repo_head(&path) {
        Ok(Some((_, commit))) => {
            let short_hash = &commit[..7.min(commit.len())];
            ch4_version = format!("{} git:{}", ch4_version, short_hash);
        }
        Ok(None) => {}
        Err(_) => {}
    }

    println!("cargo:rustc-env=CH4_VERSION={}", ch4_version);
}

fn format_file(path: &std::path::Path) {
    let path_str = path.to_str().unwrap();
    let output = Command::new("rustfmt")
        .args(&["--edition", "2018"])
        .arg(path_str)
        .output()
        .expect("failed to launch rustfmt");
    if !output.status.success() {
        panic!(
            "failed to format {}\nstdout: {}\nstderr: {}",
            path_str,
            std::str::from_utf8(&output.stdout).unwrap(),
            std::str::from_utf8(&output.stderr).unwrap(),
        );
    }
}

fn write_file(tera: &Tera, context: &Context, file_name: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let file_data = tera
        .render("main_template.rs", context)
        .expect("failed to render template");
    let file_path = std::path::Path::new(&out_dir).join(file_name);
    std::fs::write(&file_path, file_data).expect("failed to write file");
    format_file(&file_path);
}

fn write_main() {
    let tera = match Tera::new("templates/*.rs") {
        Ok(t) => t,
        Err(e) => {
            panic!("Tera parsing error(s): {}", e);
        }
    };

    for async_value in &["true", "false"] {
        let mut context = Context::new();
        context.insert("async", async_value);
        let file_name = format!(
            "{}_main.rs",
            if *async_value == "true" {
                "async"
            } else {
                "std"
            }
        );
        write_file(&tera, &context, &file_name);
    }
}