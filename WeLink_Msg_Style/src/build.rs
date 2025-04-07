#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置元数据
    use rt_helper::metadata::get_metadata;
    let product_name = get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductName");
    println!("cargo:rustc-env=PRODUCT_NAME={product_name}");
    // 设置图标资源
    let icon = format!("../assets/{}.ico", env!("CARGO_PKG_NAME"));
    winres::WindowsResource::new().set_icon(&icon).compile()?;
    Ok(())
}
