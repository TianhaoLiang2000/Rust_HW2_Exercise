pub struct Buffer<T> {
    pub Foo: Vec<T>,
}
impl<T: std::ops::Add<Output = T> + Clone + Default + std::ops::AddAssign> Buffer<T> {
    pub fn new() -> Self {
        Buffer { Foo: Vec::new() }
    }
    pub fn push(&mut self, element: T) {
        self.Foo.push(element);
    }
    pub fn sum(&self) -> T {
        let mut result = T::default();
        for i in &self.Foo {
            result += i.clone();
        }
        result
    }
}
