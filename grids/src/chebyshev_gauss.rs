use std::f64::consts::PI;

#[derive(Debug)]
struct ChebyshevGauss {
    a: f64,
    b: f64,
    order: u32,
    nodes: Vec<f64>,
    weights: Vec<f64>,
}

impl ChebyshevGauss {
    pub fn new(a: f64, b: f64, order: u32) -> Self {
        let mut nodes = Vec::new();
        let mut weights = Vec::new();
        for i in 0..order {
            nodes.push(0.5 * (b + a) + 0.5 * (b - a) * ( (2.0 * (order - i) as f64 - 1.0) / (2.0 * order as f64) * PI).cos())
        }
        for i in 0..order {
            weights.push(PI / order as f64);
        }
        ChebyshevGauss { a, b, order, nodes, weights }
    }

    // evaluates to 3.977463260506145
    pub fn compute_exp(&self, x: f64) -> f64 {
        x.exp()
    }

    pub fn integrate(&self) -> f64 {
        let mut sum = 0.0;
        for x in 0..self.order as usize {
            sum += self.weights[x] * self.compute_exp(self.nodes[x]);
        }
        sum
    }
}

mod tests {
    use super::*;

    #[test]
    fn grid_initialisation_cheby() {
        let grid = ChebyshevGauss::new(-1.0, 1.0, 8);
        println!("{:?}", grid.integrate());
    }
}

