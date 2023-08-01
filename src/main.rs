use std::process::Command;
extern crate whoami;

fn main() {
    if cfg!(target_os = "windows") {
        let name = whoami::username();
        let path = format!("C:\\Users\\{name}\\AppData\\Local\\Temp");
        Command::new("cmd")
            .args(&["/c", "rmdir", "/s", "/q", &path])
            .spawn()
            .expect("Failed to delete C:\\Users\\frane\\AppData\\Local\\Temp");
        Command::new("cmd")
            .args(&["/c", "rmdir", "/s", "/q", "C:\\Windows\\Temp"])
            .spawn()
            .expect("Failed to delete C:\\Windows\\Temp");
    } else if cfg!(target_os = "linux") {
        Command::new("rm")
            .args(&["-rf", "/tmp/*"])
            .spawn()
            .expect("Failed to delete /tmp");
    }
}



