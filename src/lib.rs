#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod neurons {
    pub trait Model<T, const N: usize> {
        fn activation_function(&self, xs: &[T; N]) -> T;
    }

    pub struct Neuron<const N: usize> {
        pub weights: [f64; N],
        pub bias: f64,
    }

    impl<const N: usize> Neuron<N> {
        pub fn new(ws: [f64; N], b: f64) -> Neuron<N> {
            Neuron {
                weights: ws,
                bias: b,
            }
        }

        pub fn set_weights(&mut self, ws: &[f64; N]) {
            for i in 0..N {
                self.weights[i] = ws[i];
            }
        }

        pub fn set_bias(&mut self, b: f64) {
            self.bias = b;
        }
    }

    pub struct Perceptron<const N: usize> {
        pub neuron: Neuron<N>,
    }

    impl<const N: usize> Model<bool, N> for Perceptron<N> {
        fn activation_function(&self, xs: &[bool; N]) -> bool{
            let mut out = 0.;

            for i in 0..N {
                if xs[i] {
                    out += self.neuron.weights[i];
                }
            }

            if out + self.neuron.bias <= 0. { false } else { true }
        }
    }
}

mod math {
    pub fn dot_product() {}
}
