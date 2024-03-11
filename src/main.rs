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
    // 通过 * 解引用 
    assert_eq!(*b, "hello");
}


#[test]
fn test_mut_borrow() {
    // 加上 mut 使变量可变，后面在函数内部进行了变化
    let mut a = String::from("hello");
    // 用 a 的引用作为参数传递给函数，而不是把所有权传给函数
    change(&mut a);
    assert_eq!(a, "hello, world");
}

fn change(some_string: &mut String){
    // 函数没有返回值，那么返回一个 ()
    some_string.push_str(", world")
}