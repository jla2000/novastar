use std::{env, fs::File, io::Write, process::Command};

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let hash = String::from_utf8(output.stdout).expect("Invalid utf-8 output");
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut f = File::create(out_dir + "/commit-hash.txt").unwrap();
    f.write_all(hash.trim().as_bytes()).unwrap();
}
