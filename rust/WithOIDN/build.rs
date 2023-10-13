use std::env;

fn main() {
  let project_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
  println!("project_dir: {}", project_dir);

  let oidn_inc_path = "../external/oidn/include/OpenImageDenoise";
  let header = format!("{}/{}/oidn.h", project_dir, oidn_inc_path);

  let bindings = bindgen::Builder::default()
    .header(header)
    .clang_arg(format!("-I{}", oidn_inc_path))
    .generate()
    .expect("Unable to generate bindings");

  let output_path = format!("{}/src/oidn", project_dir);

  std::fs::create_dir_all(&output_path).expect("Couldn't create output directory");

  bindings
    .write_to_file(format!("{}/bindings.rs", output_path))
    .expect("Couldn't write bindings!");

  println!("cargo:rustc-link-search=native={}/../external/oidn/lib", project_dir);
  println!("cargo:rustc-link-lib=dylib=OpenImageDenoise");
}