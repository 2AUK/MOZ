use ndarray::{arr1, Array1};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct GridSix {
    weight: f64,
    coord: Vec<(f64, f64)>,
}

impl GridSix {
    pub fn new() -> Self {
        /*let vec_coord = vec![
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (-1.0, 0.0, 0.0),
            (0.0, -1.0, 0.0),
            (0.0, 0.0, -1.0),
        ];*/
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
    pub fn compute(&self, coords: &(f64, f64)) -> f64 {
        let theta2 = coords.0 * 2.0;
        1.0 + (3.0 * theta2.cos())
    }

    pub fn integrate(&self) -> f64 {
        let mut sum = 0.0;
        //let mut weight_sum = 0.0;
        for coord in self.coord.iter() {
            sum += self.weight * self.compute(coord);
            //weight_sum += self.weight;
        }
        //println!("{}", weight_sum);
        4.0 * PI * sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_initialisation() {
        let grid = GridSix::new();
        println!("{:?}", grid);
        println!("{}", grid.integrate());
    }
}
