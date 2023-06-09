use std::f64::consts::PI;
use crate::lebedev_data::LDGRIDS;

#[derive(Debug)]
pub struct GridSix {
    weight: f64,
    coord: Vec<(f64, f64)>,
}

impl GridSix {
    pub fn new() -> Self {
        let ang_coord = vec![
            (0.0, 90.0),
            (180.0, 90.0),
            (90.0, 90.0),
            (-90.0, 90.0),
            (90.0, 0.0),
            (90.0, 180.0),
        ];
        GridSix {
            weight: 0.166666666666667,
            coord: ang_coord,
        }
    }

    // the integral of this function should evaluate to zero
    pub fn compute_zero_integral(&self, coords: &(f64, f64)) -> f64 {
        let theta2 = coords.1.to_radians() * 2.0;
        1.0 + 3.0 * theta2.cos()
    }
    // evaluates to -(2/3) * 2\pi
    pub fn compute_cos2theta(&self, coords: &(f64, f64)) -> f64 {
        let theta2 = coords.1.to_radians() * 2.0;
        theta2.cos()
    }
    // evaluates to 3.544907701811
    pub fn compute_y00(&self, _coords: &(f64, f64)) -> f64 {
        1.0 / (2.0 * PI.sqrt())
    }
    // should evaluate to 0
    pub fn compute_y01(&self, coords: &(f64, f64)) -> f64 {
        0.5 * (3.0 / PI).sqrt() * coords.1.to_radians().cos()
    }
    // should evaluate to 0
    pub fn compute_y02(&self, coords: &(f64, f64)) -> f64 {
        0.25 * (5.0 / PI).sqrt() * ((3.0 * coords.1.to_radians().cos() * coords.1.to_radians().cos()) - 1.0)
    }

    pub fn integrate(&self) -> (f64, f64, f64, f64, f64) {
        let (mut sum_zero, mut sum_cos2theta, mut sum_y00, mut sum_y01, mut sum_y02) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for coord in self.coord.iter() {
            sum_zero += self.weight * self.compute_zero_integral(coord);
            sum_cos2theta += self.weight * self.compute_cos2theta(coord);
            sum_y00 += self.weight * self.compute_y00(coord);
            sum_y01 += self.weight * self.compute_y01(coord);
            sum_y02 += self.weight * self.compute_y02(coord);
        }
        (4.0 * PI * sum_zero, 4.0 * PI * sum_cos2theta, 4.0 * PI * sum_y00, 4.0 * PI * sum_y01, 4.0 * PI * sum_y02)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_initialisation() {
        let grid = GridSix::new();
        println!("{:?}", grid.integrate());
    }
}
