# Molecular Ornstein-Zernike

## Current Status
- [x] Lebedev-Laikov Quadrature
- [ ] Chebyshev-Gauss Quadrature

## Installing and Running
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Since only grids are working, you can run the grid tests with `cargo test -p grids -- --nocapture`

Currently the Lebedev implementation is for order 6, and is used to integrate the function $\cos(2\theta)$, and the spherical harmonics $Y_0^0(\theta, \phi)$, $Y_1^0(\theta, \phi)$, and $Y_2^0(\theta, \phi)$. These evaluate to $-\frac{2}{3} 2\pi$, $2\pi\sqrt{\frac{1}{\pi}}$, $0$ and $0$ respectively.

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta}\cos{2\theta} = -\frac{2}{3} 2\pi$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_0^0(\theta, \psi) = 2\pi\sqrt{\frac{1}{\pi}}$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_1^0(\theta, \psi) = 0$$

$$\int_0^{2 \pi} d\psi \int_0^{\pi} d\theta \sin{\theta} Y_2^0(\theta, \psi) = 0$$

The Chebyshev-Gauss implementation is more flexible in order. The test uses order 8. Currently the function $\frac{e^x}{\sqrt{1-x^2}}$ is implemented as a test.

$$\int_{-1}^{1} dx \frac{e^x}{\sqrt{1-x^2}} \approx 3.977463260506145 $$

## References

- [Application of efficient algorithm for solving six-dimensional molecular Ornstein-Zernike equation](https://doi.org/10.1063/1.3693623)

- [Extended molecular Ornstein-Zernike integral equation for fully anisotropic solute molecules: Formulation in a rectangular coordinate system](https://doi.org/10.1063/1.4819211)

-[Integral equation algorithm for fluids of fully anisotropic molecules](https://doi.org/10.1063/1.469615)