import numpy as np
import sys
from pathlib import Path

tables_path = Path('./tables').resolve()

data_paths = sorted(list(tables_path.rglob('*.txt')))

order_list = []
const_list = []

with open('lebedev_data.rs', 'w') as f:
    f.write(f"use arrayvec::ArrayVec;\n")
    for path in data_paths:
        PHI, THETA, WEIGHTS = np.loadtxt(path, unpack=True)

        ORDER = PHI.shape[0]

        order_name = path.stem.split('_')[1]

        name = "LD" + order_name

        order_list.append(ORDER)
        const_list.append(name)

        f.write(f"pub const {name}: ArrayVec<(f64, f64, f64), {ORDER}> = ArrayVec::from([\n")
        for i in range(ORDER):
            phi = PHI[i]
            theta = THETA[i]
            weight = WEIGHTS[i]
            f.write(f"    ({theta}, {phi}, {weight}),\n")
        f.write(f"]);\n\n")

    f.write(f'static LDGRIDS: phf::Map<&\'static str, [(f64, f64, f64); N]> = phf_map! {{\n')
    for order, arr_name in zip(order_list, const_list):
        f.write(f'    "{order}" => {arr_name},\n')
    f.write(f'}};\n')