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
        fn activation_function(&self, xs: [T; N]) -> T;
    }

    pub struct Neuron<T, const N: usize> {
        weights: [f64; N],
        bias: f64,
    }

    impl<const N: usize> Neuron<T, N> {
        pub fn new(ws: [f64; N], b: f64) -> Neuron<T, N> {
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

    // pub struct Perceptron<const N: usize> {
    //     neuron: Neuron<N>,
    //     inputs: [bool; N],
    // }

    // impl<const N: usize> Model<bool, N> for Perceptron<N> {
    //     fn activation_function(neuron: Neuron<N>, xs: &[bool; N]) -> bool{
    //         let mut out = 0.;

    //         for i in 0..N {
    //             if xs[i] {
    //                 out += neuron.weights[i];
    //             }
    //         }

    //         if out + neuron.bias <= 0. { false } else { true }
    //     }
    // }
}

mod math {
    pub fn dot_product() {}
}
