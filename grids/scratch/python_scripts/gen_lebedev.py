import numpy as np
import sys

PHI, THETA, WEIGHTS = np.loadtxt("lebedev_009.txt", unpack=True)

ORDER = PHI.shape[0]

def gen_const_arr(opath, name):
    with open(opath, 'w') as f:
        f.write(f"pub const {name}: [(f64, f64, f64); {ORDER}] = [\n")
        for i in range(ORDER):
            phi = PHI[i]
            theta = THETA[i]
            weight = WEIGHTS[i]
            f.write(f"    ({phi}, {theta}, {weight}),\n")
        f.write(f"];\n")


if __name__ == "__main__":
    gen_const_arr(sys.argv[1], sys.argv[2])