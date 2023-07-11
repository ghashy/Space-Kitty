#[cfg(target_os = "windows")]
extern crate embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let target = std::env::var("TARGET").unwrap();
        if target.contains("windows") {
            embed_resource::compile("icon.rc");
        }
    }
}
