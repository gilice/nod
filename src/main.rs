use anyhow::{Context, Result};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    process::{self, Stdio},
    thread::sleep,
    time::Duration,
};

/// Build the same package twice and diffoscope
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[allow(unused_parens)]
struct Args {
    /// Package to build
    name: String,

    /// How much to wait between rebuilds (milliseconds)
    #[arg(short = 'D', long, default_value_t = 1000)]
    duration: u64,
    /// Path of nixpkgs repo
    #[arg(short, long, default_value = ".")]
    path: String,

    /// The build command to use.  
    #[arg(short, long, default_values_t = vec!["nix-build".to_string(), "@path".to_string(), "-A".to_string(), "@name".to_string(), "--keep-failed".to_string()])]
    build_command: Vec<String>,

    /// The build command to use.  
    #[arg(short = 'd', long, default_values_t = vec!["diffoscope".to_string(), "@1".to_string(), "@2".to_string(),  "--html".to_string(), "out.html".to_string()])]
    diff_command: Vec<String>,
}

lazy_static! {
    static ref OUTPATH: Regex = Regex::new("note: keeping build directory '(.*)'\n").unwrap();
}

/// Builds the package and returns the path that failed to build
fn build(args: &mut Args) -> Result<String> {
    let mut proc_1 = process::Command::new(&args.build_command[0]);
    let proc = proc_1.args(&args.build_command[1..]);
    dbg!(&proc);
    let proc_out = proc
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Could not spawn build process")?;
    dbg!(&proc_out);

    let parsed_stderr = std::str::from_utf8(&proc_out.stderr)
        .context("Trying to parse stderr of nix-build process as utf8")?;
    let e = "No match found for the build dir regex. Are you sure that the drv fails to build?";
    let captured = OUTPATH.captures(parsed_stderr).ok_or(e).unwrap();
    dbg!(&captured[1]);

    Ok(String::from(&captured[1]))
}

/// Replace each key of substitutions with its value in v
fn substitute<'a>(substitutions: &HashMap<&'a str, &'a String>, v: &mut Vec<String>) {
    for i in v.iter_mut() {
        if let Some(val) = substitutions.get(i.as_str()) {
            *i = i.replace(i.as_str(), val.as_str());
        }
    }
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    dbg!(&args);
    let build_substitutions = HashMap::from([("@name", &args.name), ("@path", &args.path)]);
    substitute(&build_substitutions, &mut args.build_command);
    dbg!(&args.build_command);

    let a = build(&mut args).context("While trying to build first package")?;
    sleep(Duration::from_millis(args.duration));
    let b = build(&mut args).context("While trying to build second package")?;

    let diff_substitutions = HashMap::from([("@1", &a), ("@2", &b)]);
    substitute(&diff_substitutions, &mut args.diff_command);
    let mut diff_proc1 = process::Command::new(&args.diff_command[0]);
    let diff_proc = diff_proc1.args(&args.diff_command[1..]);
    dbg!(&diff_proc);
    diff_proc
        .spawn()
        .context("While trying to spawn diffoscope command")?;

    println!("Done");
    Ok(())
}
