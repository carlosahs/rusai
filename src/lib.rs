#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod neurons {
    pub trait Model {
        fn activation_function();
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

    impl<const N: usize> Model for Perceptron<N> {
        fn activation_function() {
        }
    }
}

mod math {
    pub fn dot_product() {}
}
