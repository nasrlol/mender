/// ! June 01 21:59 - nasr
/// ! build tool
/// ! metaprogram
use std::fs;
use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::ffi::OsString;
use std::thread;
use std::time::Duration;

struct Target {
    cc: &'static str,
    target: &'static str,
    out: String,
    out_dir: PathBuf,
    args: Vec<OsString>,
    tests: &'static [&'static str],
    tests_dir: Vec<OsString>,
    tests_args: Vec<OsString>,
}

fn build_self() -> std::io::Result<()> {
    let dir = env::current_dir()?;

    let target = Target {
        cc: "rustc",
        target: "mender.rs",
        out: String::from("mender"),
        args: vec!["--edition=2021".into(), dir.join("mender.rs").into(), "-o".into(), dir.join("mender").into()],
        out_dir: dir,
        tests: &[],
        tests_dir: vec![],
        tests_args: vec![],
    };

    let mut build_output = Command::new(target.cc);

    for arg in target.args {
        build_output.arg(arg);
    }

    let status = build_output.status()?;
    if !status.success() {
        return Err(std::io::Error::other("Rebuilding mender failed!"));
    }

    Ok(())
}

fn build(mut target: Target) {
    if cfg!(target_os = "windows") {
        target.out.push_str(".exe");
    }

    println!("building target");

    let mut build_output = Command::new(target.cc);
    build_output.arg(format!("{} -o {}/{}", target.target, target.out_dir.display(), target.out));

    for arg in target.args {
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

    // compile
    {
        let dir = env::current_dir()?;
        let src = "compiler.cpp";
        let out = "compiler";

        let target = Target {
            cc: "g++",
            target: src,
            out: String::from("./build/compiler"),
            args: vec![dir.join(src).into(), "-o".into(), dir.join(out).into()],
            out_dir: dir,
            tests: &[],
            tests_dir: vec![],
            tests_args: vec![],
        };
        build(target);
    }

    Ok(())
}
