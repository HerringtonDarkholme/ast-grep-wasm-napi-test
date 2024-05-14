extern crate napi_build;

fn main() {
  let target = std::env::var("TARGET").unwrap();

  if target.contains("wasi") {
    cc::Build::new().file("dup.c").compile("dup");
  }
  napi_build::setup();
}
