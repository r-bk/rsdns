use std::process::Command;
use tera::{Context, Tera};

const NET_CRATES: &[&str] = &["tokio", "async-std", "smol", "std"];

fn dashes_to_underscores(s: &str) -> String {
    s.to_string().replace('-', "_")
}

fn need_crate(crate_name: &str) -> bool {
    let feature = format!("net-{crate_name}");
    let env_var = format!(
        "CARGO_FEATURE_{}",
        dashes_to_underscores(&feature).to_uppercase()
    );
    std::env::var_os(env_var).is_some()
}

fn format_file(path: &std::path::Path) -> bool {
    let path_str = path.to_str().unwrap();
    Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(path_str)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn write_file(tera: &Tera, context: &Context, file_name: &str, crate_name: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let template_name = format!("{file_name}.rs");
    let file_data = tera
        .render(&template_name, context)
        .expect("failed to render template");
    let dest_file_name = format!("{}_{}.rs", file_name, dashes_to_underscores(crate_name));
    let dest_file_path = std::path::Path::new(&out_dir).join(dest_file_name);
    std::fs::write(&dest_file_path, file_data).expect("failed to write file");
    format_file(&dest_file_path);
}

fn write_clients(tera: &Tera) {
    for crate_name in NET_CRATES {
        if !need_crate(crate_name) {
            continue;
        }

        let mut context = Context::new();
        context.insert("feature", &format!("net-{crate_name}"));
        context.insert("crate_name", crate_name);
        context.insert("crate_module_name", &dashes_to_underscores(crate_name));
        context.insert(
            "async",
            if *crate_name != "std" {
                "true"
            } else {
                "false"
            },
        );

        write_file(tera, &context, "client", crate_name);

        if *crate_name != "std" {
            write_file(tera, &context, "async_client_impl", crate_name);
        }
    }

    println!("cargo:rerun-if-changed=templates/client.rs");
    println!("cargo:rerun-if-changed=templates/async_client_impl.rs");
}

fn main() {
    let tera = match Tera::new("templates/*.rs") {
        Ok(t) => t,
        Err(e) => {
            panic!("Tera parsing error(s): {e}");
        }
    };

    write_clients(&tera);
}
