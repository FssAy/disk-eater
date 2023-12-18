fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=ui/index.css");
  println!("cargo:rerun-if-changed=ui/index.html");
  println!("cargo:rerun-if-changed=ui/style.css");
  tauri_build::build()
}
