use self_update::cargo_crate_version;

pub fn update(bin_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("yequanrui")
        .repo_name("RuTools")
        .bin_name(bin_name)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
