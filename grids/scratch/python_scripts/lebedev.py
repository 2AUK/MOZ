import numpy as np

phi, theta, weights = np.loadtxt("lebedev_009.txt", unpack=True)

order = phi.shape[0]

def cos2theta(phi, theta):
    return np.cos(2.0 * np.radians(theta))

def zero(phi, theta):
    return 1 + 3.0 * np.cos(2.0 * np.radians(theta))

def Y00(phi, theta):
    return 1.0 / (2.0 * np.sqrt(np.pi))

def Y01(phi, theta):
    return 0.5 * np.sqrt(3.0 / np.pi) * np.cos(np.radians(theta))

def Y02(phi, theta):
    return 0.25 * np.sqrt(5.0 / np.pi) * (3.0 * np.cos(np.radians(theta)) * np.cos(np.radians(theta)) - 1.0)

def integrate(f):
    sum = 0.0
    for i in range(order):
        sum += weights[i] * f(phi[i], theta[i])
    return 4.0 * np.pi * sum

print(integrate(zero), integrate(cos2theta), integrate(Y00), integrate(Y01), integrate(Y02))