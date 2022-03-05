extern crate core;

use std::env;
use std::env::args;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

type DynError = Box<dyn std::error::Error>;

#[derive(Clone, Copy)]
enum Targets {
    X86_64,
    Aarch64,
}

impl FromStr for Targets {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let target = match s {
            "x86_64-unknown-none" => Targets::X86_64,
            "aarch64-unknown-none" => Targets::Aarch64,
            _ => panic!("Unsupported architecture"),
        };
        Ok(target)
    }
}

impl Display for Targets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let target = match self {
            Targets::X86_64 => "x86_64-unknown-none",
            Targets::Aarch64 => "aarch64-unknown-none",
        };
        write!(f, "{}", target)
    }
}

#[derive(Debug, Clone)]
enum Tasks {
    Kbuild,
    Ktest,
    Image,
}

#[derive(Clone)]
struct Arguments {
    task: Tasks,
    target: Targets,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            task: Tasks::Kbuild,
            target: Targets::X86_64,
        }
    }
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1)
    }
}

fn parse_arguments(args_struct: &mut Arguments) {
    println!("{:?}", env::args().nth(1));
    args_struct.task = match env::args()
        .nth(1)
        .expect("No valid task provided, defaulting to kbuild")
        .as_str()
    {
        "kbuild" | "build" => Tasks::Kbuild,
        "ktest" | "test" => Tasks::Ktest,
        "image" => Tasks::Image,
        _ => panic!("Non valid operation provided."),
    };

    for x in env::args() {
        if x.as_str() == "--target" {
            args_struct.target = match env::args()
                    .next()
                    .expect("No valid value provided, defaulting to x86_64-unknown-none.")
                    .as_str()
                    // Validates the entered values
                {
                    "x86_64-unknown-none" => Targets::X86_64,
                    "aarch64-unkown-none" => {
                        println!("The bootloader does not currently support the aarch64 architecture. Only the kernel will be built.");
                        args_struct.task = Tasks::Kbuild;
                        Targets::Aarch64
                    }
                    _ => panic!("Architecture not supported yet, if you would like to get involved make sure to open an issue on https://github.com/InfRandomness/Arc/issues?q=is%3Aissue+is%3Aopen")
                }
        }
    }
}

fn try_main() -> Result<(), DynError> {
    let mut arguments = Arguments::default();
    parse_arguments(&mut arguments);
    match arguments.task {
        Tasks::Kbuild => build_kernel(&arguments.target),
        Tasks::Ktest => test(),
        Tasks::Image => image(&arguments.target)?,
    }
    Ok(())
}

/// Runs the kernel's test suite.
fn test() {
    const _TEST_ARGS: &[&str] = &[
        "-device",
        "isa-debug-exit,iobase=0xf4,iosize=0x04",
        "-serial",
        "stdio",
        "-display",
        "none",
        "--no-reboot",
    ];
    const _TEST_TIMEOUT_SECS: u64 = 10;

    println!("thing");
}

/// Builds the kernel.
fn build_kernel(target: &Targets) {
    println!("Compiling the kernel");
    let mut build_cmd = Command::new(env!("CARGO"));
    // Set the current dir as the kernel dir
    build_cmd.current_dir(
        project_root::get_project_root()
            .expect("An error has occurred while acquiring the project root")
            .join("../kernel"),
    );
    build_cmd.arg("build");
    build_cmd.arg("--target").arg(format!("{}", target));

    if !build_cmd.status().unwrap().success() {
        panic!("Build failed");
    }
}

/// Parses the arguments to transform the kernel into a bootable image.
fn image(target: &Targets) -> Result<(), DynError> {
    println!("Creating an image");
    build_kernel(target);
    // TODO: Make a friendlier experience by telling the path was not found
    let kernel_binary_path = {
        let path = PathBuf::from(
            args()
                .next()
                .expect("No path to the kernel binary was found."),
        );
        path.canonicalize().unwrap()
    };

    create_disk_image(&kernel_binary_path);
    Ok(())
}

fn create_disk_image(kernel_binary_path: &Path) -> PathBuf {
    //TODO: Fix this terrible hack.
    let project_root = project_root::get_project_root().expect("Unable to find a project root");

    let kernel_manifest_path = project_root.join("../kernel/Cargo.toml");
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
