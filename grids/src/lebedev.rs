use crate::lebedev_data::{get_lebedev_grid, LebedevGrid};
use ndarray::{arr1, Array, Array1, Zip};
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]

pub struct LebedevLaikovGrid {
    weights: Array1<f64>,
    //coord: Vec<(f64, f64)>,
    theta: Array1<f64>,
    phi: Array1<f64>,
}

impl LebedevLaikovGrid {
    pub fn new(grid: LebedevGrid) -> Self {
        let grid = get_lebedev_grid(grid);
        let grid_size = grid.len();
        let mut weights = Array::zeros(grid_size);
        let mut theta = Array::zeros(grid_size);
        let mut phi = Array::zeros(grid_size);
        for (i, elem) in grid.iter().enumerate() {
            weights[[i]] = elem.2;
            theta[[i]] = elem.0;
            phi[[i]] = elem.1;
        }
        LebedevLaikovGrid {
            weights,
            theta,
            phi,
        }
    }

    pub fn integrate(&self, f: fn(&f64, &f64) -> f64) -> f64 {
        let mut _out = Array::zeros(self.weights.raw_dim());
        Zip::from(&self.weights)
            .and(&self.theta)
            .and(&self.phi)
            .and(&mut _out)
            .par_for_each(|w, t, p, o| {
                *o = w * f(&t, &p);
            });

        4.0 * PI * _out.iter().sum::<f64>()
        // let grid_iter = zip(&self.weight, &self.theta, &self.phi);
        // 4.0 * PI
        //     * grid_iter
        //         .map(|(weight, (theta, phi))| weight * f(theta, phi))
        //         .sum::<f64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const PRECISION: f64 = 1e-10;

    fn one_plus_three_cos_2theta(theta: &f64, _phi: &f64) -> f64 {
        let theta2 = theta.to_radians() * 2.0;
        1.0 + 3.0 * theta2.cos()
    }

    fn cos_2theta(theta: &f64, _phi: &f64) -> f64 {
        let theta2 = theta.to_radians() * 2.0;
        theta2.cos()
    }

    fn y00(_theta: &f64, _phi: &f64) -> f64 {
        1.0 / (2.0 * PI.sqrt())
    }

    fn y01(theta: &f64, _phi: &f64) -> f64 {
        0.5 * (3.0 / PI).sqrt() * theta.to_radians().cos()
    }

    fn y02(theta: &f64, _phi: &f64) -> f64 {
        0.25 * (5.0 / PI).sqrt()
            * ((3.0 * theta.to_radians().cos() * theta.to_radians().cos()) - 1.0)
    }

    #[test]
    fn grid_initialisation() {
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD003);
        let test_grid = LebedevLaikovGrid {
            weights: arr1(&[
                0.166666666666667,
                0.166666666666667,
                0.166666666666667,
                0.166666666666667,
                0.166666666666667,
                0.166666666666667,
            ]),
            theta: arr1(&[90.0, 90.0, 90.0, 90.0, 0.0, 180.0]),
            phi: arr1(&[0.0, 180.0, 90.0, -90.0, 90.0, 90.0]),
        };
        assert_eq!(grid, test_grid);
    }

    #[test]
    fn cos_2theta_integral() {
        // evaluates to -(2/3) * 2 * pi
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(cos_2theta);
        assert_abs_diff_eq!((-2.0 / 3.0 * 2.0 * PI), integral, epsilon = PRECISION);
    }

    #[test]
    fn one_plus_three_cos_2theta_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(one_plus_three_cos_2theta);
        assert_abs_diff_eq!(0.0, integral, epsilon = PRECISION);
    }

    #[test]
    fn y00_integral() {
        // evaluates to 4.0 * pi / (2.0 * sqrt(pi))
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y00);
        assert_abs_diff_eq!((4.0 * PI / 2.0 / PI.sqrt()), integral, epsilon = PRECISION);
    }

    #[test]
    fn y01_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y01);
        assert_abs_diff_eq!(0.0, integral, epsilon = PRECISION);
    }

    #[test]
    fn y02_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y02);
        assert_abs_diff_eq!(0.0, integral, epsilon = PRECISION);
    }
}
