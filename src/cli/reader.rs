use clap::Parser;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 指定搜索的起始目录，默认从当前目录开始搜索
    #[arg(short, long, default_value_os_t=env::current_dir().unwrap())]
    dir: PathBuf,

    /// 必选项，需要查找的文件名称，关键词搜索，支持使用正则表达式
    #[arg(short, long)]
    keyword: Option<String>,

    /// 非必选项，是否递归搜索子目录，默认不递归搜索
    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    /// 非必选项，是否使用模糊查询，默认不使用
    #[arg(short, long, default_value_t = false)]
    fuzzy: bool,

    /// 非必选项，是否区分大小写，默认不区分大小写
    #[arg(short, long, default_value_t = false)]
    case_sensitive: bool,

    /// 非必选项，指定文件类型，可以指定多个文件类型，用逗号分隔
    #[arg(short, long)]
    type_file: Option<String>,

    /// 非必选项，限制文件搜索数量，默认不限制
    #[arg(short, long)]
    number: Option<usize>,

    /// 非必选项，排序方式
    #[arg(short, long)]
    sort: Option<String>,

    /// 非必选项，输出方式，开启后输出文件的详细信息，默认只输出文件名
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug)]
pub struct DestructureArgs {
    pub dir: String,
    pub keyword: String,
    pub recursive: bool,
    pub fuzzy: bool,
    pub case_sensitive: bool,
    pub type_file: String,
    pub number: usize,
    pub sort: String,
    pub verbose: bool,
}

/// 读取命令行输入，并处理可能出现的错误
pub fn read() -> DestructureArgs {
    let args = Args::parse();
    let dir = args.dir.to_string_lossy().to_string();
    let keyword = match args.keyword {
        Some(keyword) => keyword,
        None => {
            println!("请输入需要查找的文件名称！-h可查看帮助");
            std::process::exit(1);
        }
    };
    let type_file = args.type_file.unwrap_or_else(|| "".to_string());
    let number = args.number.unwrap_or_else(|| usize::MAX);
    let sort = args.sort.unwrap_or_else(|| "name".to_string());
    DestructureArgs {
        dir,
        keyword,
        recursive: args.recursive,
        fuzzy: args.fuzzy,
        case_sensitive: args.case_sensitive,
        type_file,
        number,
        sort,
        verbose: args.verbose,
    }
}
