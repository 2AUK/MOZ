import numpy as np
import matplotlib.pyplot as plt
import mpl_toolkits.mplot3d.axes3d as axes3d

N = 15

def f(x):
    return np.exp(-(5*x)**2) + x

def chebyshev_nodes(a, b, n):
    # n Chebyshev noder i intervallet [a, b]
    i = np.array(range(n))
    x = np.cos((2*i+1)*np.pi/(2*(n))) # noder over intervallet [-1,1]
    return 0.5*(b-a)*x+0.5*(b+a) # noder over intervallet [a,b]

def lagrange_interpolant(x, f):
    N = len(x)
    a = np.ones(N)
    for i in range(N):
        for j in range(N):
            if i != j:
                a[i] *= 1/(x[i] - x[j])
    def p(x_):
        result = 0.
        for i in range(N):
            term_i = a[i] * f[i]
            for j in range(N):
                if i != j:
                    term_i *= x_ - x[j]
            result += term_i
        return result
    
    return p

PHI, THETA = chebyshev_nodes(0, np.pi, 40), chebyshev_nodes(-np.pi, np.pi, 40)

R = 1.0 #np.abs(Y10(np.rad2deg(PHI), np.rad2deg(THETA)))
X = R * np.sin(PHI) * np.cos(THETA)
Y = R * np.sin(PHI) * np.sin(THETA)
Z = R * np.cos(PHI)

fig = plt.figure()
ax = fig.add_subplot(projection='3d')
plot = ax.scatter(
    X, Y, Z
    )

plt.show()

