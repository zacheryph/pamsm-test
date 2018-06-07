fn main() {
    //TODO: expand this
    println!("warning=some_message_here");
    println!("cargo:rustc-link-lib=pam");
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=pam_misc");
    }
}
