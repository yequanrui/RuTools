#[cfg(windows)]
fn main() {
    // 设置元数据
    use rt_helper::metadata::get_metadata;
    let product_name = get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductName");
    println!("cargo:rustc-env=PRODUCT_NAME={product_name}");
    let product_version = get_metadata(env!("CARGO_PKG_NAME"), "winres", "ProductVersion");
    println!("cargo:rustc-env=PRODUCT_VERSION={product_version}");
    // 设置图标资源
    winres::WindowsResource::new()
        .set_icon("../logo.ico")
        .compile()
        .expect("Failed to compile Windows resources");
}
