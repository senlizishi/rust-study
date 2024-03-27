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
     * 结构体
     * 在 Rust 中 struct 和 impl 是分开的，允许同一个数据类型有不同的行为表现，也使得结构体更容易复用
     * struct：定义了数据结构
     * impl： 则是针对某个具体结构体或枚举定义其行为。还可以实现 traits（特质）
     */
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
    pub struct Post {
        pub author: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("作者是{}", self.author)
        }
    }
    #[test]
    fn test_struct() {
        // 创建实例
        let post = Post {
            author: "Sunface".to_string(),
        };
        println!("整体信息打印：{:?}", post);
        println!("{}", post.summarize());
    }

    /**
     * 特质约束 + 泛型限制入参
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
     * 特征对象（trait objects）
     * 定义：是一种动态分发的机制，用于在不知道具体类型的情况下，通过特征来引用和操作对象。特征对象由两部分组成：一个指向某个实际对象的指针，和一个指向该对象实现的特征方法的表的指针。
     * 写法：let obj:  (&dyn Trait | Box<dyn Trait>) = ...  来表示特质对象。这个特征对象并不包含实际的对象数据，可以将其视为一个指向实现了特定特征的对象的“动态”引用
     * 使用场景：当你有一个包含不同类型对象的集合，并且你想通过相同的接口来操作这些对象
     * 限制：不是所有特征都能拥有特征对象，符合对象安全的要求才行（方法的返回类型不能是 Self，方法没有任何泛型参数）
     */
    trait Animal {
        fn speak(&self);
    }
    struct Cat;
    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }
    struct Dog;
    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
    fn make_animals_speak(animals: Vec<Box<dyn Animal>>) {
        for animal in animals.iter() {
            animal.speak();
        }
    }
    #[test]
    fn test_trait() {
        let cat = Cat;
        let dog = Dog;
        // Rust 需要在编译时知道类型占用多少空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小类型
        // 使用 Box::new(T) 的方式来创建了两个 Box<dyn Draw> 特征对象（隐式转换），特质对象在做的就是将 DST 类型转换为固定大小类型
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
        make_animals_speak(animals);
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
mod mul_thread_tests {

    /**
     * Java中的线程分为守护线程和非守护线程
     * - 守护线程：程序运行时在后台提供服务的线程，如 GC 线程，当所有非守护线程结束时，Java虚拟机会自动退出，不会等待守护线程执行完毕
     * - 非守护线程（用户线程）：在程序运行过程中执行实际任务的线程，只有在Java中确保所有非守护线程都已经执行完毕，Java虚拟机就会退出。因此当用户线程 main 创建子线程 A，在 main 结束后 A 仍然会运行。
     *
     * Rust 一旦主线程（main）完成，整个程序将结束，不论子线程是否还有未完成的工作都会结束。
     * 注意：父线程结束后，子线程仍在持续运行，直到子线程的代码运行完成或者 main 线程的结束（main 线程中创建 A 线程，A线程创建 B线程，在 main 线程没结束的前提下，A线程结束不影响 B线程运行，直到 main 线程结束才会停止）
     */
    #[test]
    fn test_create() {
        use std::thread;
        use std::time::Duration;
        let handle = thread::spawn(|| {
            for _ in 1..5 {
                println!("子线程在运行!");
                thread::sleep(Duration::from_millis(2000));
            }
        });
        // 让主线程安全、可靠地等所有子线程完成任务后，再 kill self
        handle.join().unwrap();
        println!("主线程运行完毕!");
    }

    /**
     * Java 和 Rust 在多线程编程方面采取了不同的策略和技术来保障线程安全和有效并发
     *
     * Java
     * - Synchronized Blocks 和 Lock：在多线程环境下确保对共享内存的访问是有序和线程安全的（同步性）
     * - BlockingQueue：线程间通信
     * - 并发工具类：volatile 关键字确保变量的可见性和有序性、原子类（AtomicInteger）、并发集合类（ConcurrentHashMap）
     * - ExecutorService：线程池
     * - 异步编程：Runnable 和 Callable、ExecutorService(线程池)、Future 和 CompletableFuture （后面）
     *
     *
     * Rust
     * - Mutex（互斥锁） 和 RwLock（读写锁）
     * - 通道（Channel）：实现线程间通信 （发送者 -> 通道 -> 接收者）
     * - Arc：原子引用计数智能指针，使得多个线程可以安全地共享所有权和数据，而无需担心数据竞争和生命周期问题
     */
    #[test]
    fn test_channel() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        // 通道分为同步和异步
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    /**
     * 互斥锁 Mutex,同一时间，只允许一个线程访问该值，其它线程需要等待A访问完成后才能继续
     */
    #[test]
    fn test_mutex() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        // 使用 Arc 智能指针允许锁资源在同一时刻拥有多个所有者
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // m.lock() 获取锁，获取不到则处于阻塞状态，返回结果为 Result 通过 unwrap 简化值获取，Mutex<T> 是一个智能指针
                let mut num = counter.lock().unwrap();

                *num += 1;
                // 作用域结束，锁被释放
            });
            handles.push(handle);
        }

        for handle in handles {
            // 等待所有子线程的结束
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

#[cfg(test)]
mod smart_pointers {

    /**
     * 智能指针
     *
     * 与引用的区别：
     * - 智能指针比引用更复杂的数据结构，包含比引用更多的信息，例如元数据，当前长度，最大可用长度等
     * - 引用和智能指针的另一个不同在于前者仅仅是借用了数据，而后者往往可以拥有它们指向的数据
     *
     * 智能指针往往是基于结构体实现，它与我们自定义的结构体最大的区别在于它实现了 Deref 和 Drop 特征，这也是智能之处
     * - Deref：可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码（常见于隐式转换、引用归一化）
     * - Drop：允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
     *
     */
    #[test]
    fn test_box() {
        let a = 3;
        // Box<T> 智能指针将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据
        let b = Box::new(a);
        // 需要手动解引用
        let c = *b + 1;
        assert_eq!(c, 4);
    }

    /**
     * 通过引用计数的方式（通过记录一个数据被引用的次数来确定该数据是否正在被使用。当引用次数归零时，就代表该数据不再被使用，因此可以被清理释放），允许一个数据资源在同一时刻拥有多个所有者
     * - RC 适用于单线程
     * - Arc 适用于多线程
     *
     * 注意：
     * - Rc/Arc 是不可变引用，你无法修改它指向的值，如果要修改，需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
     * - 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
     */
    #[test]
    fn test_rc() {
        use std::rc::Rc;
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        //  a 和 b 是共享了底层的字符串 s,这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            // 由于变量 c 在语句块内部声明，当离开语句块时它会因为超出作用域而被释放，所以引用计数会减少 1，事实上这个得益于 Rc<T> 实现了 Drop 特征
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        // a、b、c 三个智能指针引用计数都是同样的，并且共享底层的数据，因此打印计数时用哪个都行
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
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
     * Self 与 self 的区别
     */
    trait Draw {
        // 这里之所以使用 Self ，是因为 Draw 是一个 trait 可以被多个类型实现，Self 则代表其具体的类型
        fn draw(&self) -> Self;
    }

    #[derive(Clone)]
    struct Button;
    impl Draw for Button {
        // self指代的就是当前的实例对象，Self 指的是当前的类型（Button）
        fn draw(&self) -> Self {
            return self.clone();
        }
    }
}
