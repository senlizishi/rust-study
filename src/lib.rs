use std::error::Error;
use std::fs;
mod math;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        // 使用模式匹配
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 使用 Result 来返回
        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> 特征对象，它表示函数返回一个类型，该类型实现了 Error 特征，这样我们就无需指定具体的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 如果结果是 Ok(T)，则把 T 赋值给 f，如果结果是 Err(E)，则返回该错误，所以 ? 特别适合用来传播错误
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod type_tests {

    /**
     * String 与 &str
     */
    #[test]
    fn test_string() {
        // 右侧是字符串字面量，左侧 a 实际上是 &'static str 类型的值，即指向静态存储区的不可变字符串引用
        let a = "hello";

        // String::from 方法接收一个字符串字面量，并基于这个字面量创建一个新的、可变的 String 实例。该 String 存储在堆中
        let b = String::from("hello");
        assert_eq!(b, "hello");

        // 所有权转移（move），当值的所有权从 b 转移（move）到 c 后。Rust 会认为 b 不再有效 （注意：如果是字面量则只是拷贝，不会失效）
        let c = b;
        // 借用
        let d = &c;
        assert_eq!(d, "hello"); // 通过 * 解引用，这里写 b 也行，因为 assert_eq 会自动解引用

        // &mut 可变引用,可改变值
        let mut e = String::from("hello");
        change(&mut e);
        assert_eq!(e, "hello, world");
    }
    fn change(some_string: &mut String) {
        // 函数没有返回值，那么返回一个 ()
        some_string.push_str(", world")
    }

    /**
     * 数组 array
     */
    #[test]
    fn test_array() {
        // 数组存在栈中，一旦创建后大小不能改变
        let a = [3; 5]; // 数组长度为 5，每个元素都是 3。
        println!("{:#?}", a);

        // 数组切片,切片是对数组或 Vector 等底层数据结构的引用，它并不拥有数据，而是引用数据的一部分
        let slice: &[i32] = &a[1..3]; // 这里表示从索引 1 开始，到索引 3 结束的切片
        println!("{:#?}", slice);

        // 使用标准库 std 中 array 模块提供的 from_fn 函数来初始化数组
        // from_fn 接收一个闭包作为参数，这个闭包会为数组的每个索引生成相应的值。
        // _i: 闭包参数 _i 表示数组的索引，这里使用了下划线 _ 前缀表示我们虽然接收了这个参数，但在闭包内并未使用它
        let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
        println!("{:#?}", array);
    }

    /**
     * 动态数组 vector
     */
    #[test]
    fn test_vector() {
        // 数组适合于固定长度的场景，而 Vector 则适合于动态长度及存储数据量大的场景，如果容量不足就会触发 vector 扩容
        let v: Vec<i32> = Vec::new();
        v.push(1);
        println!("{:#?}", v);

        // 使用 vec![] 宏创建时可以有初始化值
        let mut v1 = vec![1, 2, 3];
        let third: &i32 = &v[2];
        println!("第三个元素是 {}", third);

        // 基于模式匹配取值
        match v.get(2) {
            Some(third) => println!("第三个元素是 {third}"),
            None => println!("not find！"),
        }
    }

    // 枚举

    /**
     * 结构体和特征
     */
    #[test]
    fn test_struct_and_trait() {
        let post = Post {
            author: "Sunface".to_string(),
        };
        println!("整体信息打印：{:?}", post); // 使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出
        println!("{}", post.summarize());
    }
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    // 结构属性和方法定义是分离的
    #[derive(Debug)]
    pub struct Post {
        pub author: String,
    }
    // 实现特征
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("作者是{}", self.author)
        }
    }
}

#[cfg(test)]
mod ohter_tests {

    /**
     * 生命周期
     */
    #[test]
    fn test_lifecycle() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }
    // 使用 'a 进行生命周期标注，告诉编译器返回的引用生命周期是 x 和 y 作用域的重合部分，也就是 x 和 y 中生命周期最小的那部分
    // 标记的生命周期只是为了提供信息给编译器，这样编译器就拥有充分的信息来确保我们的操作是内存安全的
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    /**
     * package -> crate -> mod  use
     */
    #[test]
    fn test_mod() {
        // src/main.rs 和 src/lib.rs 被称为包根(crate root)，这两个文件的内容形成了一个模块 crate
        use crate::math::*; //  也可以使用 super 关键字来表达路径 eg. use super::math::*;
        let result = arithmetic::add(5, 10);
        println!("The sum is: {}", result);
    }

    /**
     * 函数式编程-闭包
     */
    #[test]
    fn test_closure() {
        let x = 1;
        // 函数式编程：闭包是一种匿名函数,它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
        let sum = |y| x + y;
        assert_eq!(3, sum(2));
        // 无参且有方法体写法
        let print_x = || {
            println!("muuuu.....");
            x
        };
        assert_eq!(1, print_x());
    }

    /**
     * 智能指针
     */
    #[test]
    fn test_smart_pointer() {
        // 通过 Box<T> 来创建一个智能指针，Box<T> 是指针，它指向了堆上的数据
        let a = Box::new(3);
        // 隐式地调用了 Deref 对智能指针 a 进行了解引用
        println!("a = {}", a);
        // 需要手动解引用
        let b = *a + 1;
        // a 持有的智能指针将在作用域结束（main 函数结束）时，被释放掉，这是因为 Box<T> 实现了 Drop 特征
    }
}
