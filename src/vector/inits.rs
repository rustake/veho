pub fn init<T, F>(size: usize, f: F) -> Vec<T> where
    F: Fn(usize) -> T
{ (0..size).map(|i| f(i)).collect() }

pub fn iso<T>(size: usize, value: T) -> Vec<T> where
    T: Copy
{ vec![value; size] }