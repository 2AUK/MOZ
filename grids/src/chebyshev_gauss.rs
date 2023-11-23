use ndarray::{arr1, Array, Array1, Zip};
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub struct ChebyshevGauss {
    nodes: Array1<f64>,
    weights: Array1<f64>,
}

impl ChebyshevGauss {
    pub fn new(a_limit: f64, b_limit: f64, order: usize) -> Self {
        let mut nodes = Array::zeros(order);
        let mut weights = Array::zeros(order);
        for i in 0..order {
            nodes[[i]] = 0.5 * (b_limit + a_limit)
                + 0.5
                    * (b_limit - a_limit)
                    * ((2.0 * (order - i) as f64 - 1.0) / (2.0 * order as f64) * PI).cos()
        }
        for i in 0..order {
            weights[[i]] = PI / order as f64;
        }
        ChebyshevGauss { nodes, weights }
    }

    pub fn integrate(&self, f: fn(&f64) -> f64) -> f64 {
        let mut _out = Array::zeros(self.weights.raw_dim());
        Zip::from(&self.weights)
            .and(&self.nodes)
            .and(&mut _out)
            .par_for_each(|w, n, o| {
                *o = w * f(&n);
            });

        _out.iter().sum::<f64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const PRECISION: f64 = 1e-10;

    fn compute_exp(x: &f64) -> f64 {
        x.exp()
    }

    #[test]
    fn grid_initialisation_chebyshev_check_weights() {
        let grid = ChebyshevGauss::new(-1.0, 1.0, 4);
        let test_grid = ChebyshevGauss {
            weights: arr1(&[
                0.7853981633974483,
                0.7853981633974483,
                0.7853981633974483,
                0.7853981633974483,
            ]),
            nodes: arr1(&[
                -0.9238795325112867,
                -0.3826834323650898,
                0.3826834323650897,
                0.9238795325112867,
            ]),
        };
        assert_abs_diff_eq!(test_grid.weights, grid.weights, epsilon = PRECISION);
    }

    #[test]
    fn grid_initialisation_chebyshev_check_nodes() {
        let grid = ChebyshevGauss::new(-1.0, 1.0, 4);
        let test_grid = ChebyshevGauss {
            weights: arr1(&[
                0.7853981633974483,
                0.7853981633974483,
                0.7853981633974483,
                0.7853981633974483,
            ]),
            nodes: arr1(&[
                -0.9238795325112867,
                -0.3826834323650898,
                0.3826834323650897,
                0.9238795325112867,
            ]),
        };
        assert_abs_diff_eq!(test_grid.nodes, grid.nodes, epsilon = PRECISION);
    }

    #[test]
    fn compute_exp_integral() {
        // evaluates to 3.977463260506145
        let grid = ChebyshevGauss::new(-1.0, 1.0, 5810);
        let integral = grid.integrate(compute_exp);
        assert_abs_diff_eq!(3.977463260506145, integral, epsilon = PRECISION);
    }
}
