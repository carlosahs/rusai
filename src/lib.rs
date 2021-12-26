#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Perceptron<const N: usize> {
    weights: [f64; N],
    bias: f64,
}
