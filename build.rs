fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _child = std::process::Command::new("cargo")
    //     .arg("build")
    //     .arg("-p")
    //     .arg("elam-rs")
    //     .spawn()
    //     .expect("failed to start wasm build");
    //
    // let mut target_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR")?.into();
    // target_dir.push("target");
    // target_dir.push(::std::env::var("PROFILE")?);
    //
    // let mut elam_old = target_dir.clone();
    // elam_old.push("elam_rs.dll");
    //
    // let mut elam_new = target_dir.clone();
    // elam_new.push("elam_rs.sys");
    //
    // println!("{:?}", elam_old.to_str());
    // std::fs::rename(elam_old, elam_new)?;
    Ok(())
}
