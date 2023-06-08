# Molecular Ornstein-Zernike

## Current Status
Working on implementing angular quadratures

## Installing and Running
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Since only grids are working, you can run the grid tests with `cargo test -p grids -- --nocapture`

Currently the Lebedev implementation is for order 6, and is used to integrate the function $\cos(2\theta)$, and the spherical harmonics $Y_0^0(\theta, \phi)$, $Y_1^0(\theta, \phi)$, and $Y_2^0(\theta, \phi)$. These evaluate to $-\frac{2}{3} 2\pi$, $2\pi\sqrt{\frac{1}{\pi}}$, $0$ and $0$ respectively.

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta}\cos{2\theta} = -\frac{2}{3} 2\pi$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_0^0(\theta, \psi) = 2\pi\sqrt{\frac{1}{\pi}}$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_1^0(\theta, \psi) = 0$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_2^0(\theta, \psi) = 0$$

The Chebyshev-Gauss implementation is a work in progress.