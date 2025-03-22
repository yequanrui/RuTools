use crate::i18n;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand, Package};

/// 获取Cargo元数据
pub fn get_cargo_metadata() -> Metadata {
    MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .expect(i18n::get("read_metadata_failed"))
}

/// 获取指定包
pub fn get_package(package_name: &str) -> Package {
    get_cargo_metadata()
        .packages
        .into_iter()
        .find(|package| package.name == package_name)
        .expect(i18n::get("package_not_found"))
}

/// 获取指定包的指定资源的指定参数
pub fn get_metadata(package_name: &str, resource_name: &str, param_name: &str) -> String {
    get_package(package_name).metadata[resource_name][param_name]
        .as_str()
        .unwrap()
        .to_string()
}
