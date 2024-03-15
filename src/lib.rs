use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // 使用 Result 来返回
        Ok(Config { query, file_path })
    }
}


// Box<dyn Error> 特征对象，它表示函数返回一个类型，该类型实现了 Error 特征，这样我们就无需指定具体的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // 如果结果是 Ok(T)，则把 T 赋值给 f，如果结果是 Err(E)，则返回该错误，所以 ? 特别适合用来传播错误
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
