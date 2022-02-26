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

    let should_boot = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--no-run" => true,
            other => panic!("Unexpected argument {}", other),
        }
    } else {
        false
    };

    let bios = create_disk_image(&kernel_binary_path);

    if !should_boot {
        println!("Created disk image at {}", bios.display());
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd.arg("-drive")
        .arg(format!("format=raw,file={}", bios.display()));
    run_cmd.args(RUN_ARGS);

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().or(1));
    }
    Ok(())
}

fn create_disk_image(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd.arg("--target-dir").arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd.arg("--out-dir").arg(kernel_binary_path.parent().unwrap());
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
