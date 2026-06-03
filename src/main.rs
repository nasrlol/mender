///! June 01 21:59 - nasr
///! build script for the kernel
///! generates an iso in the ./build folder
///! setup code for setting up the dev environment

use std::env;
use std::process::Command;

const OUTPUT_DIR: &str = "./isofiles";
const FILE_NAME: &str = "vorthur-kernel.bin";
const ISO_FILE: &str = "vorthur.iso";

// Grub configuration
const GRUB_TIMEOUT: u8 = 5;
const GRUB_DEFAULT: u8 = 0;
const GRUB_ENTRY: &str = "vorthur";

struct AnvilConfig {
    timeout: u8,
    default: u8,
    entry: String,
    iso_directory: String,
    iso_file_name: String,
}

fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}

fn cargo_command() -> Command {
    let mut command = Command::new("rustup");
    command.args(["run", "nightly", "cargo"]);
    command
}

impl AnvilConfig {
    fn new(iso_directory: String, iso_file_name: String) -> Self {
        AnvilConfig {
            timeout: GRUB_TIMEOUT,
            default: GRUB_DEFAULT,
            entry: GRUB_ENTRY.to_string(),
            iso_directory,
            iso_file_name,
        }
    }
}

fn build() {
    if is_debug_build() {
        println!("INFO: Starting kernel build process...");
        let build_status = cargo_command()
            .args(&[
                "build",
                "-Z", "json-target-spec",
                "-Z", "build-std=core,alloc",
                "--target-dir", OUTPUT_DIR,
            ])
            .status();

        match build_status {
            Ok(status) if status.success() => {
                println!("INFO: Cargo build completed successfully.");
            }
            other => panic!("[ERROR] Build failed or cargo crashed: {:?}", other),
        }
        println!("INFO: Running validation tests...");
        let test_status = cargo_command().args(["test"]).status();

        if test_status.is_err() || !test_status.unwrap().success() {
            let _ = cargo_command().args(["clean"]).status();
            panic!("[ERROR] Tests failed! Cleaned target directory.");
        }

        println!("INFO: Generating ISO...");
        build_iso();
    }
}

fn build_iso() {
    let iso_status = Command::new("grub-mkrescue")
        .args(&["-o", ISO_FILE, OUTPUT_DIR])
        .status();

    match iso_status {
        Ok(status) if status.success() => {
            println!("INFO: ISO creation completed successfully.");
        }
        Ok(_) => panic!("[ERROR] ISO creation failed"),
        Err(e) => panic!("[ERROR] Failed to execute grub-mkrescue: {}", e),
    }
}

fn setup_dev_env() {
    if is_debug_build() {
        println!("INFO: Setting up development environment...");

        let hook_status = Command::new("git")
            .args(&["config", "core.hooksPath", ".githooks"])
            .status();

        match hook_status {
            Ok(status) if status.success() => {
                println!("INFO: Git hooks configured successfully.");
                // TODO: Ensure .githooks have correct executable permissions
            }
            _ => println!("WARNING: Failed to configure git hooks"),
        }

        println!("cargo:rerun-if-changed=build.rs");
    } else {
        println!("INFO: Not in debug profile, skipping setup.");
    }
}

fn run_vm() {
    if is_debug_build() {
        // Check if qemu is available
        let qemu_check = Command::new("qemu-system-x86_64")
            .args(&["--version"])
            .status();

        match qemu_check {
            Ok(status) if status.success() => {
                println!("INFO: QEMU found, proceeding with VM launch...");
                // 1. Generate the ISO file using grub based on the ./isofiles folder
                // 2. Compile the kernel
                // 3. Move the kernel to the correct place
                // 4. Start the VM with the kernel

                build_iso();
                println!("INFO: Starting QEMU with kernel...");
                // TODO: Launch QEMU with appropriate arguments
            }
            _ => panic!("Install QEMU first. Install the correct version while you're at it."),
        }
    }
}

fn load_config_file(_path: &str) {
    // Placeholder for ACL parsing.
    // The build tool currently only needs the file to exist so the CLI can start.
}

fn build_grub_config(config: &AnvilConfig) -> String {
    format!(
        "set timeout={}\nset default={}\nmenuentry \"{}\" {{\n  # TODO: Add boot configuration\n}}\n",
        config.timeout, config.default, config.entry
    )
}


fn main() {
    let args: Vec<String> = env::args().collect();

				let default_location: &str = "./mender.acl";
				load_config_file(default_location);

    if args.len() < 2 {
        eprintln!("Usage: cargo run [run|build|test|setup]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "build" => {
            println!("INFO: Building kernel...");
            build();
        }
        "run" => {
            println!("INFO: Running VM...");
            run_vm();
        }
        "test" => {
            println!("INFO: Running tests...");
            // TODO: Implement test command
            let test_status = cargo_command().args(["test"]).status();

            match test_status {
                Ok(status) if status.success() => println!("INFO: Tests passed."),
                _ => panic!("[ERROR] Tests failed"),
            }
        }
        "setup" => {
            setup_dev_env();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: build, run, test, setup");
        }
    }
}
