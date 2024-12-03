pub fn lock() {
    std::process::Command::new("cmd")
        .args(["/C", "rundll32.exe", "user32.dll,LockWorkStation"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
