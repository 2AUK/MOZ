import numpy as np
import matplotlib.pyplot as plt

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

x = chebyshev_nodes(0, np.pi, 6)
print(x)
