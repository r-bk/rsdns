use zyn::TokenStream;

#[path = "templates/async_client_impl.rs"]
mod async_client_impl_template;

#[path = "templates/client.rs"]
mod client_template;

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

fn write_file(tokens: TokenStream, file_name: &str, crate_name: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_file_name = format!("{}_{}.rs", file_name, dashes_to_underscores(crate_name));
    let dest_file_path = std::path::Path::new(&out_dir).join(dest_file_name);
    let syntax_tree: zyn::syn::File =
        zyn::syn::parse2(tokens).expect("failed to parse generated tokens");
    let pretty = prettyplease::unparse(&syntax_tree);
    std::fs::write(&dest_file_path, pretty).expect("failed to write file");
}

fn write_clients() {
    for crate_name in NET_CRATES {
        if !need_crate(crate_name) {
            continue;
        }
        let is_async = *crate_name != "std";

        write_file(
            client_template::render(crate_name, is_async),
            "client",
            crate_name,
        );

        if is_async {
            write_file(
                async_client_impl_template::render(crate_name),
                "async_client_impl",
                crate_name,
            );
        }
    }

    println!("cargo:rerun-if-changed=templates/client.rs");
    println!("cargo:rerun-if-changed=templates/async_client_impl.rs");
}

fn main() {
    write_clients();
}
