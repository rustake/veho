#[cfg(test)]
mod tests {
    #[test]
    fn alpha_test() {
        let tuples = [
            (1, 10.0),
            (2, 20.0),
            (3, 30.0),
        ];

        let result: (Vec<&i32>, Vec<&f64>) = (&tuples).into_iter().map(|&(ref a, ref b)| (a, b)).unzip();
        println!("{:?}", result);
    }
}