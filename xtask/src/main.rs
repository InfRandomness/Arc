extern crate core;

use std::env;
use std::env::Args;
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
    match task.as_deref() {
        Some("image") => image(std::env::args().skip(2))?,
        _ => print_help(),
    }
    Ok(())
}

fn image(mut args: Skip<Args>) -> Result<(), DynError> {
    // TODO: Make a friendlier experience by telling the path was not found
    let kernel_binary_path = {
        let path = PathBuf::from(
            args.next()
                .expect("No path to the kernel binary was found."),
        );
        path.canonicalize().unwrap()
    };

    /*
    let target = args.next().map_or_else(
        || {
            println!("No target value was provided, defaulting to x86_64_unknown_uefi");
            "x86_64-unknown-uefi"
        },
        |image_str| match image_str.as_ref() {
            "x86_64-arc" | "x86_64-unknown-none" => "x86_64-unknown-none",
            "aarch64-unknown-none" | "aarch64-arc.json" => {
                panic!("The bootloader currently doesn't support ARM targets.");
            }
            _ => panic!("Unsupported architecture."),
        },
    );
    */

    create_disk_image(&kernel_binary_path);
    Ok(())
}

fn create_disk_image(kernel_binary_path: &Path) -> PathBuf {
    //TODO: Fix this terrible hack.
    let project_root = project_root::get_project_root().expect("Unable to find a project root");

    let kernel_manifest_path = PathBuf::from(project_root).join("../kernel/Cargo.toml");
    let bootloader_manifest_path =
        bootloader_locator::locate_bootloader("bootloader", Some(&kernel_manifest_path))
            .unwrap_or_else(|err| {
                panic!(
                    "an unrecoverable error occurred while locating the bootloader: \n{}",
                    err
                )
            });

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    // TODO: Fix this too.
    build_cmd.arg("--target-dir").arg(
        kernel_manifest_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("target"),
    );
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("Build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));

    if !disk_image.exists() {
        panic!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        );
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
