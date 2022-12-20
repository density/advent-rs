use std::env;
use std::fs::{copy, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use cargo::ops::NewOptions;
use cargo::ops::VersionControl::NoVcs;
use cargo::Config;

fn main() {
    let (year, problem_number) = extract_args();

    let proj_dir = create_package(year, problem_number);
    write_dependencies(&proj_dir);
    create_input_file(&proj_dir);
    copy_template(&proj_dir);
}

fn write_dependencies(proj_dir: &Path) {
    let mut toml_path = PathBuf::from(proj_dir);
    toml_path.push("Cargo.toml");

    OpenOptions::new()
        .append(true)
        .open(&toml_path)
        .expect("Couldn't find Cargo.toml")
        .write_all("hymns = { path = \"../../hymns\" }".as_bytes())
        .expect("Couldn't write to Cargo.toml.");
}

fn extract_args() -> (u8, u8) {
    let mut args = env::args();
    args.next(); // skip arg 0

    let year = args
        .next()
        .and_then(|y_str| {
            if y_str.len() == 2 {
                y_str.parse().ok()
            } else {
                None
            }
        })
        .expect("Expected a 2 digit year.");

    let problem_number = args
        .next()
        .and_then(|p_str| p_str.parse().ok())
        .expect("Invalid problem number.");

    (year, problem_number)
}

fn create_package(year: u8, problem_number: u8) -> PathBuf {
    let formatted_name = format!("aoc{:02}-{:02}", year, problem_number);

    let package_dir: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        &format!("20{:02}", year),
        &formatted_name,
    ]
    .iter()
    .collect();

    let options = NewOptions::new(
        Some(NoVcs),
        true,
        false,
        package_dir.clone(),
        Some(formatted_name),
        None,
        None,
    )
    .unwrap();
    cargo::ops::new(&options, &Config::default().unwrap()).expect("Project already exists.");

    package_dir
}

fn create_input_file(proj_dir: &Path) {
    let input_file: PathBuf = [proj_dir.to_str().unwrap(), "input.txt"].iter().collect();
    File::create(input_file).expect("Could not create input file.");
}

fn copy_template(proj_dir: &Path) {
    let src: PathBuf = [env!("CARGO_MANIFEST_DIR"), "template.rs"].iter().collect();
    let dst: PathBuf = [proj_dir.to_str().unwrap(), "src", "main.rs"]
        .iter()
        .collect();
    copy(src, dst).expect("Could not copy template.");
}
