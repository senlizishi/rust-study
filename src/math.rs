// math 模块，该模块下又有 arithmetic 和 geometry 两个子模块，模块内部有定义好公开的函数
pub mod arithmetic {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // 私有的外部无法调用
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}