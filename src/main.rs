use std::env;
use std::process;
use rust_study::Config;

fn main() {
    // unwrap_or_else中如果 Result 是 OK 则返回 Ok 的值，否则返回 Err 的值
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // 终结进程
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 我们并不关注 run 返回的 Ok 值，因此只需要用 if let 去匹配是否存在错误即可
    if let Err(e) = rust_study::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
