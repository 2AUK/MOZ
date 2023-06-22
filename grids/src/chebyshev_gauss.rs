use std::f64::consts::PI;
use std::iter::zip;

#[derive(Debug, PartialEq)]
pub struct ChebyshevGauss {
    nodes: Vec<f64>,
    weights: Vec<f64>,
}

impl ChebyshevGauss {
    pub fn new(a_limit: f64, b_limit: f64, order: u32) -> Self {
        let mut nodes = Vec::new();
        let mut weights = Vec::new();
        for i in 0..order {
            nodes.push(
                0.5 * (b_limit + a_limit)
                    + 0.5
                        * (b_limit - a_limit)
                        * ((2.0 * (order - i) as f64 - 1.0) / (2.0 * order as f64) * PI).cos(),
            )
        }
        for _i in 0..order {
            weights.push(PI / order as f64);
        }
        ChebyshevGauss { nodes, weights }
    }

    pub fn integrate(&self, f: fn(&f64) -> f64) -> f64 {
        let grid_iter = zip(&self.weights, &self.nodes);
        grid_iter
            .map(|(weight, node)| weight * f(node))
            .sum::<f64>()
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
            weights: vec![0.7853981633974483, 0.7853981633974483, 0.7853981633974483, 0.7853981633974483],
            nodes: vec![-0.9238795325112867, -0.3826834323650898, 0.3826834323650897, 0.9238795325112867],
        };
        assert_abs_diff_eq!(
            test_grid.weights.as_slice(),
            grid.weights.as_slice(),
            epsilon = PRECISION
        );
    }

    #[test]
    fn grid_initialisation_chebyshev_check_nodes() {
        let grid = ChebyshevGauss::new(-1.0, 1.0, 4);
        let test_grid = ChebyshevGauss {
            weights: vec![0.7853981633974483, 0.7853981633974483, 0.7853981633974483, 0.7853981633974483],
            nodes: vec![-0.9238795325112867, -0.3826834323650898, 0.3826834323650897, 0.9238795325112867],
        };
        assert_abs_diff_eq!(
            test_grid.nodes.as_slice(),
            grid.nodes.as_slice(),
            epsilon = PRECISION
        );
    }

    #[test]
    fn compute_exp_integral() {
        // evaluates to 3.977463260506145
        let grid = ChebyshevGauss::new(-1.0, 1.0, 5810);
        let integral = grid.integrate(compute_exp);
        assert_abs_diff_eq!(3.977463260506145, integral, epsilon = PRECISION);
    }
}
