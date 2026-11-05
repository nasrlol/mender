/// ! June 01 21:59 - nasr
/// ! build tool
/// ! metaprogram
use std::fs;
use std::env;
use std::process::Command;
use std::ffi::OsString;
use std::thread;
use std::time::Duration;

fn build_self() -> std::io::Result<()> {

    
    let target = Target {

	cc: "rustc",
	target: "mender.rs",
	out: "mender",
	out_dir: env::current_dir()?,
	args: vec!["--edition=2021".into(), dir.join("mender.rs").into(), "-o".into(), dir.join("mender").into()],
	tests: &[],
	tests_dir: vec![],
	tests_args: vec![],
    }

    const CC: &str = "rustc";
    let dir = 

    let args: Vec<OsString> = 

    let mut build_output = Command::new(CC);

    for arg in args {
        build_output.arg(arg);
    }

    let status = build_output.status()?;
    if !status.success() {
        return Err(std::io::Error::other("Rebuilding mender failed!"));
    }

    Ok(())
}

struct Target {
    cc: &str,
    target: &str,
    out: String,
    out_dir: &str,
    args: Vec<OsString>,
    tests: &[&str],
    tests_dir: Vec<OsString>,
    tests_args: Vec<OsString>,
    
}

fn build(target: Target) {
    if cfg!(target_os = "windows") {
        target.out.append(".exe");
    }

    println!("building target");

    let mut build_output = Command::new(COMPILER);
    build_output.arg(format!("{} -o {}/{}", TARGET, TARGET_DIRECTORY, OUT));

    for arg in COMMON_ARGS {
        build_output.arg(arg);
    }
}

fn main() -> std::io::Result<()> {
    const DIRECTORIES: &[&str] = &["build", "tests"];

    for dir in DIRECTORIES {
        fs::create_dir_all(dir)?;
        println!("building {}", dir);
    }

    // compile self if modified
    {
        if fs::metadata("mender.rs")?.modified()? > fs::metadata("mender")?.modified()? {
            println!("building self");
            build_self()?;
        }
    }

    Ok(())
}
