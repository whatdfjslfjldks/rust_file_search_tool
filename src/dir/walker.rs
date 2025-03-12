use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::cli::reader;

/// 搜索函数，根据是否递归调用不同的搜索方法
pub fn walker(args: reader::DestructureArgs) -> Result<Vec<String>, Box<dyn Error>> {
    if args.recursive {
        let result = recursive_search(&args.dir, &args)?;
        Ok(result)
    } else {
        let result = no_recursive_search(&args.dir, &args)?;
        Ok(result)
    }
}

/// 递归搜索函数
fn recursive_search(path: &str, args: &reader::DestructureArgs) -> Result<Vec<String>, Box<dyn Error>> {
    let mut target_type: Vec<String> = Vec::new();
    let entries = read_dir_with_error_handling(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_result = recursive_search(&path.to_string_lossy().to_string(), args)?;
            target_type.extend(sub_result);
        }

        if filter_file(&path, args) {
            target_type.push(path.to_string_lossy().to_string());
        }

        if target_type.len() >= args.number {
            break;
        }
    }

    sort_results(&mut target_type, &args.sort);
    Ok(target_type)
}

/// 非递归搜索函数
fn no_recursive_search(path: &str, args: &reader::DestructureArgs) -> Result<Vec<String>, Box<dyn Error>> {
    let mut target_type: Vec<String> = Vec::new();
    let entries = read_dir_with_error_handling(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if filter_file(&path, args) {
            target_type.push(path.to_string_lossy().to_string());
        }

        if target_type.len() >= args.number {
            break;
        }
    }

    sort_results(&mut target_type, &args.sort);
    Ok(target_type)
}

/// 读取目录并处理错误
fn read_dir_with_error_handling(path: &str) -> Result<fs::ReadDir, Box<dyn Error>> {
    let entries = fs::read_dir(path)?;
    Ok(entries)
}

/// 过滤文件，根据关键词、文件类型、大小写敏感等条件进行过滤
fn filter_file(path: &Path, args: &reader::DestructureArgs) -> bool {
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy().to_string();
        let file_ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if!args.type_file.is_empty() {
            let file_types: Vec<&str> = args.type_file.split(',').collect();
            if!file_types.contains(&file_ext) {
                return false;
            }
        }

        if args.keyword.is_empty() {
            return true;
        }

        if args.fuzzy {
            if args.case_sensitive {
                return file_name_str.contains(&args.keyword);
            } else {
                return file_name_str.to_lowercase().contains(&args.keyword.to_lowercase());
            }
        } else {
            let regex_pattern = if args.case_sensitive {
                Regex::new(&format!("^{}$", args.keyword)).unwrap()
            } else {
                Regex::new(&format!("(?i)^{}$", args.keyword)).unwrap()
            };
            return regex_pattern.is_match(&file_name_str);
        }
    }
    false
}

/// 对搜索结果进行排序
fn sort_results(results: &mut Vec<String>, sort: &str) {
    match sort.to_lowercase().as_str() {
        "asc" => results.sort(),
        "desc" => results.sort_by(|a, b| b.cmp(a)),
        _ => {}
    }
}
