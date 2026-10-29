use std::path::PathBuf;
use std::process::Command;

pub fn open_file_dialog() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        linux_open_dialog()
    }
    #[cfg(not(target_os = "linux"))]
    {
        rfd::FileDialog::new()
            .add_filter("MATLAB Script", &["m"])
            .pick_file()
    }
}

pub fn save_file_dialog() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        linux_save_dialog()
    }
    #[cfg(not(target_os = "linux"))]
    {
        rfd::FileDialog::new()
            .add_filter("MATLAB Script", &["m"])
            .set_file_name("script.m")
            .save_file()
    }
}

#[cfg(target_os = "linux")]
fn linux_open_dialog() -> Option<PathBuf> {
    if which("zenity") {
        let output = Command::new("zenity")
            .args(&["--file-selection", "--file-filter=MATLAB Script (*.m) | *.m", "--title=Pilih File NumLite"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    if which("kdialog") {
        let output = Command::new("kdialog")
            .args(&["--getopenfilename", ".", "*.m|MATLAB Script"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    if which("yad") {
        let output = Command::new("yad")
            .args(&["--file", "--file-filter=*.m | MATLAB Script"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    rfd::FileDialog::new()
        .add_filter("MATLAB Script", &["m"])
        .pick_file()
}

#[cfg(target_os = "linux")]
fn linux_save_dialog() -> Option<PathBuf> {
    if which("zenity") {
        let output = Command::new("zenity")
            .args(&["--file-selection", "--save", "--file-filter=MATLAB Script (*.m) | *.m", "--filename=script.m", "--title=Simpan File NumLite"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    if which("kdialog") {
        let output = Command::new("kdialog")
            .args(&["--getsavefilename", ".", "*.m|MATLAB Script"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    if which("yad") {
        let output = Command::new("yad")
            .args(&["--file", "--save", "--file-filter=*.m | MATLAB Script", "--filename=script.m"])
            .output()
            .ok()?;
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
        return None;
    }
    rfd::FileDialog::new()
        .add_filter("MATLAB Script", &["m"])
        .set_file_name("script.m")
        .save_file()
}

#[cfg(target_os = "linux")]
fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
