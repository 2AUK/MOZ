use ndarray::Array1;
use std::f64::consts::PI;

pub struct Radial {
    pub real_nodes: Array1<f64>,
    pub fourier_nodes: Array1<f64>,
    pub radius: f64,
    pub num_nodes: usize,
    pub real_spacing: f64,
    pub fourier_spacing: f64,
}

impl Radial {
    pub fn new(radius: f64, num_nodes: usize) -> Self {
        let real_spacing = radius / num_nodes as f64;
        let fourier_spacing = 2.0 * PI / (2.0 * num_nodes as f64 * real_spacing);
        Radial {
            real_nodes: Array1::range(0.5, num_nodes as f64, 1.0) * real_spacing,
            fourier_nodes: Array1::range(0.5, num_nodes as f64, 1.0) * fourier_spacing,
            radius,
            num_nodes,
            real_spacing,
            fourier_spacing,
        }
    }
}
