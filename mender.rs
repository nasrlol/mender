/// ! June 01 21:59 - nasr
/// ! build tool
/// ! metaprogram
use std::fs;
use std::env;
use std::process::Command;
use std::ffi::OsString;

fn build_self() -> std::io::Result<()> {
    const CC: &str = "rustc";
    let dir = env::current_dir()?;

    let args: Vec<OsString> = vec!["--edition=2021".into(), dir.join("mender.rs").into(), "-o".into(), dir.join("mender").into()];

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

fn main() -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    const OUT: &str = "foo.exe";

    #[cfg(target_os = "linux")]
    const OUT: &str = "foo";

    const DIRECTORIES: &[&str] = &["build", "tests"];
    const COMPILER: &str = "gcc";
    const TARGET: &str = "bar.c";
    const TARGET_DIRECTORY: &str = "source";
    const COMMON_ARGS: &[&str] = &["-g", "-wno-unused"];

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

    // handle target after
    {
        let mut build_output = Command::new(COMPILER);
        build_output.arg(format!("{} -o {}/{}", TARGET, TARGET_DIRECTORY, OUT));

        for arg in COMMON_ARGS {
            build_output.arg(arg);
        }
    }

    Ok(())
}
