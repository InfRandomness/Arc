extern crate core;

use core::panicking::panic;
use std::env;
use std::env::Args;
use std::fmt::format;
use std::iter::Skip;
use std::path::{Path, PathBuf};
use std::process::Command;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1)
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("image") => image(std::env::args().skip(2))?,
        _ => print_help(),
    }
    Ok(())
}

const RUN_ARGS: &[&str] = &["--no-reboot", "-s"];

fn image(mut args: Skip<Args>) -> Result<(), DynError> {
    // TODO: Make a friendlier experience by telling the path was not found
    let kernel_binary_path = {
        let path = PathBuf::from(args.nth(0).unwrap());
        path.canonicalize().unwrap()
    };

    let target_image: String = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--target" => arg,
            other => panic!("Unexpected argument {}", other),
        }
    } else {
        "x86_64-arc-uefi".to_string()
    };

    create_disk_image(&kernel_binary_path, target_image);
    Ok(())
}

fn create_disk_image(kernel_binary_path: &Path, target_json: String) -> PathBuf {
    let kernel_manifest_path = PathBuf::from("../kernel");
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader", Some(&kernel_manifest_path)).unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd.arg("--target-dir").arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd.arg("--out-dir").arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--target").arg(target_json);
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("Build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path.parent().unwrap().join(format!("boot-bios-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        panic!("Disk image does not exist at {} after bootloader build", disk_image.display());
    }
    disk_image
}

fn print_help() {
    eprintln!(
        r#"Tasks:
image            builds an image of the kernel and the bootloader
"#
    )
}
