
pub struct Counter {
    pub count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn get_next(&mut self, margin: usize, size: usize) -> i32 {
        let pos = self.count + margin;
        self.count += size;
        return pos as i32;
    }
}