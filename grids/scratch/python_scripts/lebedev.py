import numpy as np
import matplotlib.pyplot as plt
import mpl_toolkits.mplot3d.axes3d as axes3d

phi, theta, weights = np.loadtxt("tables/lebedev_013.txt", unpack=True)

order = phi.shape[0]

def cos2theta(phi, theta):
    return np.cos(2.0 * np.radians(theta))

def zero(phi, theta):
    return 1 + 3.0 * np.cos(2.0 * np.radians(theta))

def Y00(phi, theta):
    return 1.0 / (2.0 * np.sqrt(np.pi))

def Y10(phi, theta):
    return 0.5 * np.sqrt(3.0 / np.pi) * np.cos(np.radians(theta))

def Y20(phi, theta):
    return 0.25 * np.sqrt(5.0 / np.pi) * (3.0 * np.cos(np.radians(theta)) * np.cos(np.radians(theta)) - 1.0)

def integrate(f):
    sum = 0.0
    for i in range(order):
        sum += weights[i] * f(phi[i], theta[i])
    return 4.0 * np.pi * sum

print(integrate(zero), integrate(cos2theta), integrate(Y00), integrate(Y10), integrate(Y20))

THETA, PHI = np.deg2rad(phi), np.deg2rad(theta)#np.meshgrid(np.deg2rad(theta), np.deg2rad(phi))
R = np.abs(Y20(np.rad2deg(PHI), np.rad2deg(THETA)))
X = R * np.sin(PHI) * np.cos(THETA)
Y = R * np.sin(PHI) * np.sin(THETA)
Z = R * np.cos(PHI)

fig = plt.figure()
ax = fig.add_subplot(projection='3d')
plot = ax.scatter(
    X, Y, Z
    )

plt.show()

