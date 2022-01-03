#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod neurons {
    pub struct Neuron<const N: usize> {
        pub weights: [f64; N],
        pub bias: f64,
    }

    impl<const N: usize> Neuron<N> {
        pub fn out(&self, xs: &[bool; N]) -> bool {
            if self.dot_product(xs) + self.bias <= 0. {
                false;
            }

            true
        }

        fn dot_product(&self, xs: &[bool; N]) -> f64 {
            let mut dot = 0.;

            for i in 0..N {
                if xs[i] {
                    dot += self.weights[i];
                }
            }

            dot
        }
    }
}
