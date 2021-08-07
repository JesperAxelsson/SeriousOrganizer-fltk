
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment(i32),
    Decrement(i32),
}