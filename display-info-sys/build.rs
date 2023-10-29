use std::env;
use std::path::Path;

fn main() {
    let generated = bindgen::builder()
        .header("include/display-info.h")
        .allowlist_item(r"^di_.*$")
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .c_naming(false)
        .disable_name_namespacing()
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");

    generated.write_to_file(dest_path).unwrap();
}
