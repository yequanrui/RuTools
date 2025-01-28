use crate::i18n;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand, Package};
use serde_json::Value;

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
    let metadata = get_cargo_metadata();
    metadata
        .packages
        .into_iter()
        .filter(|package| package.name == package_name)
        .next()
        .expect(i18n::get("package_not_found"))
}

/// 获取指定包的元数据
///
/// 返回一个JSON对象，取数据用json\["key"\]的方式
pub fn get_package_metadata(package_name: &str) -> Value {
    get_package(package_name).metadata
}

/// 获取指定包的指定资源的指定参数
pub fn get_metadata(package_name: &str, resource_name: &str, param_name: &str) -> String {
    get_package_metadata(package_name)[resource_name][param_name].to_string()
}
