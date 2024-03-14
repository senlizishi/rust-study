
mod math; // 引入了 math 模块
pub use math::arithmetic;

fn main() {
    println!("Hello, world!");
}

/* 
Rust 基于所有权来管理内存，编译器在编译时会根据一系列规则进行检查

所有权原则：
- Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
- 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
- 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

*/ 
#[test]
fn test_string() {
    // 表示字符串字面量，字符串字面量是在编译时就已经知道的固定内容，它们存储在程序的只读数据段（read-only data section），而非堆上
    let a= "hello";
    // 创建的 String 类型存储在堆上，是动态可变的
    let b = a.to_string();
    assert_eq!(b, "hello");
}


#[test]
fn test_owner_ship_move() {
    let a = String::from("hello");
    // 当值的所有权从 a 转移（move）到 b 后。Rust 会认为 a 不再有效 （注意：如果是字面量则只是拷贝，不会失效）
    let b = a;
    assert_eq!(b, "hello");
}

#[test]
fn test_owner_ship_borrow() {
    let a = String::from("hello");
    // 借(borrow)的方式,创建了一个指向 a 的引用，当引用离开作用域后，其指向的值也不会被丢弃
    let b = &a;
    // 通过 * 解引用，这里写 b 也行，因为 assert_eq 会自动解引用
    assert_eq!(*b, "hello");
}


#[test]
fn test_mut_borrow() {
    let mut a = String::from("hello");
    // 借用时需要借用 &mut a (可变引用)，才可改变值
    change(&mut a);
    assert_eq!(a, "hello, world");
}

fn change(some_string: &mut String){
    // 函数没有返回值，那么返回一个 ()
    some_string.push_str(", world")
}

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

// Rust 的对象定义和方法定义是分离的
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}
impl User {
    // 构造器函数（没有 self 的函数被称之为关联函数，因为是函数，所以不能用 . 的方式来调用，我们需要用 :: 来调用）
    pub fn new(active: bool, username: String,sign_in_count: u64) -> Self {
        User { active, username,sign_in_count }
    }
    pub fn sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
}

#[test]
fn test_struct() {
    // let mut user1 = User {
    //     username: String::from("test"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // 采用构造函数创建对象
    let user1 = User::new(true, String::from("test"),1);
    println!("整体信息打印：{:?}", user1); // 使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出
    println!("用户名为： {}", user1.username);
    print!("值为：{}",user1.sign_in_count())

}

#[test]
fn test_module() {
    
    // 可以用 use 简化路径
    let result = arithmetic::add(5, 10);
    println!("The sum is: {}", result);

    // 使用绝对路径调用方法
    let square_area = math::geometry::area_of_square(4.0);
    println!("The area of a square with side length 4 is: {}", square_area);
}