fn main() {
    // Hanya jalankan kode ini jika kita build di Windows
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }

    // Memberitahu Cargo agar build ulang jika icon.ico berubah
    println!("cargo:rerun-if-changed=icon.ico");
}
