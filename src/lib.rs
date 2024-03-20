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

// Box<dyn Error> 特质对象，它表示函数返回一个类型，该类型实现了 Error 特质，这样我们就无需指定具体的错误类型
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
     * 数组适合于固定长度的场景，而 Vector 则适合于动态长度及存储数据量大的场景，如果容量不足就会触发 vector 扩容
     */
    #[test]
    fn test_vector() {
        // 使用 vec![] 宏创建时可以有初始化值
        let v = vec![1, 2, 3];
        println!("{:#?}", v);
        let third: &i32 = &v[2];
        println!("第三个元素是 {}", third);

        let mut v1: Vec<i32> = Vec::new();
        v1.push(1);

        // 基于模式匹配取值
        match v1.get(0) {
            // Option<T> 被包含在标准库中，因此不需要将其显式引入作用域。它的成员 Some 和 None 也是如此，无需使用 Option:: 前缀就可直接使用 Some 和 None
            // Option 目的是不再担心会错误的使用一个 null，为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 Option<T> 中。接着，当使用这个值时，必须明确的处理值为空的情况。
            Some(one) => println!("第一个元素是 {one}"),
            None => println!("not find！"),
        }
    }

    /**
     * HashMap
     * 可否作为 key 需要该类型实现 Hash 和 Eq 特质。
     * 哈希函数的作用在于把所有 key 映射到唯一的哈希值，方便快速查找。高安全性的哈希函数能够避免哈希碰撞，但是可能会损失一定的性能。
     */
    #[test]
    fn test_hashmap() {
        // 需要手动引入
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        match scores.get(&team_name) {
            Some(target) => println!("值是 {target}"), // 注意这里是取引用
            None => println!("not find！"),
        }
        // 直接取值写法
        let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
        println!("值是 {score}")
    }

    /**
     * 泛型
     * Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。类似于 Java 中泛型擦除
     */
    #[test]
    fn test_generics() {
        let number_list = vec![34, 50, 25, 100, 65];
        print_list(&number_list);
        let char_list = vec!['y', 'm', 'a', 'q'];
        print_list(&char_list)
    }
    // 给 T 泛型实现 Display 特质，使其能够打印
    fn print_list<T: std::fmt::Display>(list: &[T]) {
        for i in list {
            println!("{i}");
        }
    }

    /**
     * 结构体（包含特质与泛型)
     */
    #[test]
    fn test_struct_and_trait() {
        // 这一步称为创建结构体实例，简称为实例
        let post = Post {
            author: "Sunface".to_string(),
        };
        println!("整体信息打印：{:?}", post); // 使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出
        println!("{}", post.summarize());
    }
    // 特质定义
    pub trait Summary {
        fn summarize(&self) -> String;

        // 特质也可以有默认实现
        fn summarize2(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub trait Display {}
    #[derive(Debug)] // 特征派生语法，被标记的结构体会自动实现特质代码。总之，derive 派生出来的是 Rust 默认给我们提供的特征，在开发过程中极大的简化了自己手动实现相应特征的需求
     // 结构体定义
    pub struct Post {
        pub author: String,
    }
     // 结构体定于与方法定义是分离的
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("作者是{}", self.author)
        }
    }

    /**
     * 特质约束--入参必须具有约束中的特质
     */
    // 特质约束（语法糖写法）,这种语法糖写法无法保证多个参数类型相同，只能保证他们实现的特质相同
    pub fn sugar(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }
    // 这种写法保证入参类型相同
    pub fn real<T: Summary + Display>(item: &T, item2: &T) {
        println!("{}", item.summarize());
    }
    // where 写法，当特质约束太复杂时可以使用 where 写法
    pub fn real_where<T, U>(item: &T, item2: &U)
    where
        T: Summary + Display,
        U: Summary + Display,
    {
        println!("{}", item.summarize());
    }

    /**
     * 枚举
     */
    #[test]
    fn test_enum() {
        let m1 = Message::Quit;
        let m2 = Message::Move { x: 1, y: 1 };
        println!("{:?}", m1);
    }
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
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
        // a 持有的智能指针将在作用域结束（main 函数结束）时，被释放掉，这是因为 Box<T> 实现了 Drop 特质
    }
}
