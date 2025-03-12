mod cli;
mod dir;

fn main() {
    // 读取用户命令行输入
    let args = cli::reader::read();
    // 根据用户输入内容查找文件
    let a = dir::walker::walker(args);
    match a {
        Ok(val) => {
            println!("文件的位置是： {:#?}", val)
        }
        Err(e) => {
            println!("发生错误： {}，请输入-h查看帮助", e)
        }
    }
}
