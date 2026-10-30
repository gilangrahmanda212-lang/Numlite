fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
    // Tambahkan baris ini:
    println!("cargo:rerun-if-changed=icon.ico");
}