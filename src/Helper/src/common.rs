use std::fs::{copy, create_dir_all, metadata, read_dir, read_to_string, remove_file, write, File};
use std::io::{stdin, BufRead, ErrorKind, Result};
use std::path::Path;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;

use crate::console::{error, info, success, warning};
use crate::i18n::get;

/// 控制台暂停，等待用户按下Enter键
pub fn pause() {
    stdin().lock().lines().next();
}

/// 等待n秒后退出
pub fn wait_for_exit(n: u64) {
    println!("{}{n}{}", get("exit_tips_1"), get("exit_tips_2"));
    sleep(Duration::from_secs(n));
    exit(0);
}

/// 创建文件，已存在就覆盖
pub fn create_file(file_name: &str) -> File {
    match File::create(file_name) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            // 文件已存在，删除并重新创建
            remove_file(file_name).unwrap_or_else(|_| panic!("{}", get("delete_failed")));
            File::create(file_name).unwrap_or_else(|_| panic!("{}", get("create_failed")))
        }
        Err(e) => {
            // 打印其他错误信息
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}

/// 打开浏览器访问url地址
pub fn open_url(url: &str) {
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        // 对url中的&使用^转义，如果不做处理，&后面的参数无法取到
        cmd.args(["/C", &format!("start {}", url.replace("&", "^&"))]);
        match cmd.output() {
            Ok(output) => {
                // 检查输出以确定命令是否成功
                if !output.status.success() {
                    eprintln!("{}", get("open_url_failed"));
                }
            }
            Err(e) => {
                eprintln!("{}: {e}", get("run_command_failed"));
            }
        }
    }
}

/// 备份文件
pub fn make_backup(og_path: String, version: String, is_install: bool) -> Result<bool> {
    let destination = format!("{}.bak.{}", og_path, version);
    fn install(og_path: String, destination: String) {
        println!("- {}", get("backup_tips_1"));
        match copy(&og_path, &destination) {
            Ok(_) => println!("-- {}", success(get("backup_succeeded"))),
            Err(err) => eprintln!("-- {}: {err}", error(get("backup_failed"))),
        }
    }
    if metadata(&destination).is_ok() {
        println!("-- {}", get("backup_tips_2"));
        match copy(&destination, &og_path) {
            Ok(_) => println!("-- {}", success(get("restore_succeeded"))),
            Err(err) => eprintln!("-- {}: {err}", error(get("restore_failed"))),
        }
        match remove_file(&destination) {
            Ok(_) => println!("-- {}", success(get("delete_succeeded"))),
            Err(err) => eprintln!("-- {}: {err}", error(get("delete_failed"))),
        }
        if is_install {
            install(og_path, destination);
            Ok(true)
        } else {
            Ok(false)
        }
    } else if is_install {
        install(og_path, destination);
        Ok(true)
    } else {
        println!("-- {}", info(get("no_backup")));
        Ok(false)
    }
}

/// 复制文件，复制前目标若存在，将其移除
pub fn copy_file(from: &Path, to: &Path) -> Result<()> {
    if to.exists() {
        remove_file(to)?;
    }
    copy(from, to)?;
    Ok(())
}

/// 递归复制文件/文件夹

pub fn copy_func(from: &Path, to: &Path) -> Result<()> {
    if from.is_dir() {
        for entry in read_dir(from)? {
            let entry = entry?;
            let from_path = entry.path();
            let to_path = to.join(from_path.file_name().unwrap());
            if from_path.is_dir() {
                create_dir_all(&to_path)?;
                copy_func(&from_path, &to_path)?;
            } else {
                copy_file(&from_path, &to_path)?;
            }
        }
    } else {
        copy_file(from, to)?;
    }
    Ok(())
}

/// 复制资源
pub fn copy_res(from_dir: &Path, to_dir: &Path) {
    if let Err(e) = copy_func(from_dir, to_dir) {
        match e.kind() {
            ErrorKind::NotFound => eprintln!("{}", get("not_found")),
            ErrorKind::PermissionDenied => eprintln!("{}", get("permission_denied")),
            _ => eprintln!("Error: {:?}", e),
        }
    } else {
        println!("-- {}", success(get("copy_succeeded")));
    }
}

/// 写入文件
pub fn write_to(file_path: String, replaced_content: String, content: String) {
    if replaced_content == content {
        println!("-- {}", info(get("skip_write")));
        return;
    }
    write(file_path, replaced_content).unwrap_or_else(|_| panic!("{}", get("write_failed")));
    println!("-- {}\n", info(get("write_succeeded")));
}

/// 替换文本，替换前判断本是否存在
pub fn replace(origin: &str, from: &str, to: &str) -> String {
    if !origin.contains(from) {
        println!(
            "-- {}{}",
            get("skip_replace_tips_1"),
            warning(get("skip_replace_tips_3"))
        );
        return origin.to_owned();
    }
    origin.replace(from, to)
}

/// 通过替换插入文本
pub fn replace_str(file_path: String, replace_str: &str, replace_target: &str, is_restore: bool) {
    match read_to_string(&file_path) {
        Ok(content) => {
            println!("- {}", get("replace_tips_1"));
            if content.contains(replace_str) {
                if is_restore {
                    println!("- {}", get("replace_tips_2"));
                    let replaced_content = content.replace(replace_str, "");
                    write(file_path, replaced_content)
                        .unwrap_or_else(|_| panic!("{}", get("replace_failed")));
                } else {
                    println!(
                        "-- {}{}",
                        get("skip_replace_tips_1"),
                        warning(get("skip_replace_tips_4"))
                    );
                }
            } else if is_restore {
                println!("-- {}", info(get("no_replace")));
            } else {
                let replaced_content =
                    content.replace(replace_target, &(replace_str.to_owned() + replace_target));
                match write(file_path, replaced_content) {
                    Ok(()) => println!("-- {}", success(get("replace_succeeded"))),
                    Err(err) => eprintln!("-- {}: {:?}", error(get("replace_failed")), err),
                }
            }
        }
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            eprintln!(
                "- {}{}",
                get("skip_replace_tips_2"),
                warning(get("skip_replace_tips_3"))
            );
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

/// 判断是否匹配目标版本a.b.c
///
/// 完全匹配返回true，皆返回false
pub fn compare_version(origin_version: &str, target_version: &str) -> bool {
    let origin_nums: Vec<&str> = origin_version.split(".").collect();
    let target_nums: Vec<&str> = target_version.split(".").collect();
    if origin_nums[0] == target_nums[0] {
        if origin_nums[1] == target_nums[1] {
            if origin_nums[2] == target_nums[2] {
                println!(
                    "{}{}\n",
                    get("compare_version_tips_1"),
                    success(get("compare_version_tips_2"))
                );
                return true;
            } else {
                println!(
                    "{}{}\n",
                    get("compare_version_tips_1"),
                    warning(get("compare_version_tips_3"))
                );
            }
        } else {
            println!(
                "{}{}{}\n",
                get("compare_version_tips_1"),
                warning(get("compare_version_tips_4")),
                info(origin_version)
            );
        }
    } else {
        println!(
            "{}{}{}\n",
            get("compare_version_tips_1"),
            error(get("compare_version_tips_5")),
            info(origin_version)
        );
    }
    false
}

/// 判断两个版本的大小，用于判断是否有新版本
///
/// 大于返回true，小于等于false
pub fn judge_version(origin_version: &str, target_version: &str) -> bool {
    let parse_failed = get("parse_failed");
    let origin_nums: Vec<&str> = origin_version.split(".").collect();
    let target_nums: Vec<&str> = target_version.split(".").collect();
    let origin_ints: Vec<i8> = origin_nums
        .into_iter()
        .map(|s| s.parse::<i8>().expect(parse_failed))
        .collect();
    let target_ints: Vec<i8> = target_nums
        .into_iter()
        .map(|s| s.parse::<i8>().expect(parse_failed))
        .collect();
    if origin_ints[0] == target_ints[0] {
        if origin_ints[1] == target_ints[1] {
            target_ints[2] > origin_ints[2]
        } else {
            target_ints[1] > origin_ints[1]
        }
    } else {
        target_ints[0] > origin_ints[0]
    }
}
