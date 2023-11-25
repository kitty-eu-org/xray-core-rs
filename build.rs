use std::env;
use std::process::Command;
use anyhow::{Context, Result};

fn main() -> Result<()>{
    let root_dir = env::current_dir().unwrap();
    let root_dir_str = root_dir.to_string_lossy();
    println!("Root directory: {}", root_dir_str);

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("out_dir: {:?}", out_dir);
    println!("root_dir: {:?}", root_dir);
    let platform = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let output = if platform == "windows" {
        format!("{}/libxray.lib", out_dir)
    } else {
        format!("{}/libxray.a", out_dir)
    };
    println!("output: {}", output);
    let go_src = format!("{}/xray-go/main", root_dir.to_string_lossy());
    println!("go_src: {}", go_src);
    // let mut command = Command::new("go");
    // command.arg("build")
    //     .args(["-C", "xray-go"])
    //     .args(["-o", output.as_str()])
    //     .arg("-buildmode=c-archive")
    //     .arg("-trimpath")
    //     .arg("-ldflags")
    //     .args(["'-s -w'", &go_src]);
    let mut command = Command::new("go");
    command.arg("build")
        .arg("-C")
        .arg("xray-go")
        .arg("-o")
        .arg(output.as_str())
        .arg("-buildmode=c-archive")
        .arg("-trimpath")
        .arg("-ldflags")
        .arg("-s -w")
        .arg(&go_src)
        .output()
        .expect("Failed to execute 'go build' command");
    let command_str = format!("{:?}", command);
    println!("command_str: {}", command_str);
    let command_output = command
        .spawn()
        .context("fail spawning go build")?
        .wait_with_output()?;
    if !command_output.status.success() {
        let stdout = String::from_utf8_lossy(&command_output.stdout);
        println!("Command output:\n{}", stdout);
    }
    else{
        let stderr = String::from_utf8_lossy(&command_output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=xray");
    println!("cargo:rustc-link-lib=resolv");
    Ok(())

}