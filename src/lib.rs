#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod neurons {
    pub struct Perceptron<const N: usize> {
        weights: [f64; N],
        bias: f64,
    }

    impl<const N: usize> Perceptron<N> {
        pub fn out(&self, xs: &[bool; N]) -> bool {
            return false;
        }
    }
}
