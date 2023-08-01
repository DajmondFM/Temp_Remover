use std::process::Command;
extern crate whoami;

fn main() {
    let name = whoami::username();
    let path = format!("C:\\Users\\{name}\\AppData\\Local\\Temp");
    Command::new("cmd")
        .args(&["/c", "rmdir", "/s", "/q", "{}", &path])
        .spawn()
        .expect("Failed to delete C:\\Users\\frane\\AppData\\Local\\Temp");
    Command::new("cmd")
        .args(&["/c", "rmdir", "/s", "/q", "C:\\Windows\\Temp"])
        .spawn()
        .expect("Failed to delete C:\\Windows\\Temp");
}



