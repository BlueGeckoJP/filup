use std::process::Command;

fn main() {
    Command::new("pnpm")
        .args(["run", "build"])
        .current_dir("./src-ts/")
        .status()
        .unwrap();
}
