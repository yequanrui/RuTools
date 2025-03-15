# RuTools

> Tools written by Rust.

| 包名               | 描述（带*️⃣号的代码收录在本代码仓，带#️⃣号的是发布包收录）     | 最新版本                          |
|:-----------------|:-------------------------------------|:------------------------------|
| [WeLink]         | 华为云WeLink，数字化办公协作平台                  | [7.50.3.569][WeLink-Download] |
| [MT]             | #️⃣微软SDK中用于生成签名文件和目录的工具，仅用于自动获取管理员权限 | 5.2.3790.2076                 |
| Client           | *️⃣Tauri客户端                          | 1.0.0                         |
| Helper           | *️⃣一些三方库的封装方法                        | 1.0.9                         |
| WeLink_Themes    | *️⃣WeLink主题替换工具                      | 1.0.9                         |
| WeLink_Msg_Style | *️⃣WeLink显眼包替换工具                     | 1.1.9                         |

[MT]: https://learn.microsoft.com/zh-cn/windows/win32/sbscs/mt-exe "点击跳转"

[WeLink]: https://www.huaweicloud.com/product/welink "点击跳转"

[WeLink-Download]: https://www.huaweicloud.com/product/welink-download.html "点击跳转"

## 如何使用

- 发布包直接收录在代码库的，直接克隆仓库到本地即可使用
- 代码收录在仓库的在可在仓库的[Release](https://github.com/yequanrui/RuTools/releases)中下载对应版本的发布包

## 如何反馈

- 提交[Issue](https://github.com/yequanrui/RuTools/issues)
- 联系[作者](mailto:yequanrui@qq.com)

## 如何贡献

### WeLink_Msg_Style

1. 在`WeLink_Msg_Style/src/version`目录下新建一个要适配版本的rs脚本文件（注：一个脚本适配一个正式版本，故脚本名用正式版本命名，而适配的版本要完整的，用来判断。例如：脚本名`v7_17.rs`，适配版本`7.17.16`，如果当前版本配置与之前版本的一致，则重复使用配置，脚本名上体现，如`v7_17_49.rs`）
2. 定义公共函数`main`（例如：`pub fn main(install_path: String, install_version: String, is_install: bool) {}`）
3. 编写`main`函数代码，主要是`WeLink`源码替换的部分，具体可参考`v7_17.rs`，除此之外的公共代码基本不会动
4. 在`WeLink_Msg_Style/src/main.rs`主脚本里的版本映射变量`compatible_versions`插入键值对（例如：`compatible_versions.insert("7.19.6", version::v7_18_19::main);`）
5. 修改`Cargo.toml`配置文件
    1. 红版WeLink配置：`WeLink_Msg_Style/WeLink_Desktop/Cargo.toml`
    2. 蓝版WeLink配置：`WeLink_Msg_Style/WeLink/Cargo.toml`
    3. `version`值加1（逢10进1）
    4. `ProductVersion`值为适配的WeLink版本
6. 控制台执行`.\build.bat`，编译打包成发布包，发布包在根目录下的`dist`目录

### WeLink_Themes

1. 主题资源文件适配WeLink最新版本，资源文件在项目WeTools维护，与此项目建议在同级目录下共同维护，方便打包脚本将发布包同步复制到主题资源文件目录下，与主题一起打包发布
2. 修改`data.rs`，如果有新增适配的WeLink界面文件，在`assets_list()`方法中增加该界面文件的相对路径，例如`"plugin/im/dist/multiwindow.html",`，如果是红色WeLink独有的可移到`spec_list`变量的`if`块中，蓝版WeLink独有的可移到`spec_list`变量的`else`块中
3. 修改`Cargo.toml`配置文件
    1. 红版WeLink配置：`WeLink_Themes/WeLink_Desktop/Cargo.toml`
    2. 蓝版WeLink配置：`WeLink_Themes/WeLink/Cargo.toml`
    3. `version`值加1（逢10进1）
    4. `ProductVersion`值为适配的WeLink版本
4. 控制台执行`.\build.bat`，编译打包成发布包，发布包在根目录下的`dist`目录
