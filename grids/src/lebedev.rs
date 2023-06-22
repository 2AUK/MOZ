use std::f64::consts::PI;
use std::iter::zip;
use crate::lebedev_data::{LebedevGrid, get_lebedev_grid};

#[derive(Debug, PartialEq)]

pub struct LebedevLaikovGrid {
    weight: Vec<f64>,
    coord: Vec<(f64, f64)>,
}

impl LebedevLaikovGrid {
    pub fn new(grid: LebedevGrid) -> Self {
        let grid = get_lebedev_grid(grid);
        let mut weight = Vec::new();
        let mut coord = Vec::new();
        for elem in grid{
            weight.push(elem.2);
            coord.push((elem.0, elem.1));
        }
        LebedevLaikovGrid { weight, coord }
    }

    pub fn integrate(&self, f: fn(&f64, &f64) -> f64) -> f64 {
        let grid_iter = zip(&self.weight, &self.coord);
        4.0 * PI * grid_iter.map(|(weight, (theta, phi))| weight * f(theta, phi)).sum::<f64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const PRECISION: f64 = 1e-7;

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
            weight: vec![0.166666666666667, 0.166666666666667, 0.166666666666667, 0.166666666666667, 0.166666666666667, 0.166666666666667],
            coord: vec![
                (90.0, 0.0),
                (90.0, 180.0),
                (90.0, 90.0),
                (90.0, -90.0),
                (0.0, 90.0),
                (180.0, 90.0),
            ]
        };
        assert_eq!(grid, test_grid);
    }

    #[test]
    fn cos_2theta_integral() {
        // evaluates to -(2/3) * 2 * pi
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(cos_2theta);
        assert_abs_diff_eq!((-2.0 / 3.0 * 2.0 * PI), integral, epsilon=PRECISION);
    }

    #[test]
    fn one_plus_three_cos_2theta_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(one_plus_three_cos_2theta);
        assert_abs_diff_eq!(0.0, integral, epsilon=PRECISION);
    }

    #[test]
    fn y00_integral() {
        // evaluates to 4.0 * pi / (2.0 * sqrt(pi))
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y00);
        assert_abs_diff_eq!((4.0 * PI / 2.0 / PI.sqrt()), integral, epsilon=PRECISION);
    }

    #[test]
    fn y01_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y01);
        assert_abs_diff_eq!(0.0, integral, epsilon=PRECISION);
    }

    #[test]
    fn y02_integral() {
        // evaluates to 0
        let grid = LebedevLaikovGrid::new(LebedevGrid::LD131);
        let integral = grid.integrate(y02);
        assert_abs_diff_eq!(0.0, integral, epsilon=PRECISION);
    }
}
