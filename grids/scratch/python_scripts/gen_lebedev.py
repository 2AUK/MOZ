import numpy as np
import sys
from pathlib import Path

tables_path = Path('./tables').resolve()

data_paths = sorted(list(tables_path.rglob('*.txt')))

offset_list = []
const_list = []

with open('lebedev_data.rs', 'w') as f:
    f.write(f"use phf::phf_map;\n")
    total = 0
    for path in data_paths:
        PHI, THETA, WEIGHTS = np.loadtxt(path, unpack=True)
        ORDER = PHI.shape[0]
        total += ORDER

    offset_index = 0

    f.write(f"pub const LD_DATA: [(f64, f64, f64); {total}] = [\n")
    for path in data_paths:
        PHI, THETA, WEIGHTS = np.loadtxt(path, unpack=True)

        ORDER = PHI.shape[0]

        order_name = path.stem.split('_')[1]

        name = "LD" + order_name

        offset_list.append((offset_index, offset_index + ORDER))
        offset_index += ORDER
        const_list.append(name)
        
        for i in range(ORDER):
            phi = PHI[i]
            theta = THETA[i]
            weight = WEIGHTS[i]
            f.write(f"    ({theta}, {phi}, {weight}),\n")

    f.write(f"];\n\n")

    f.write(f'static LDGRIDS: phf::Map<&\'static str, (usize, usize)> = phf_map! {{\n')
    for offset, arr_name in zip(offset_list, const_list):
        f.write(f'    "{arr_name}" => {offset},\n')
    f.write(f'}};\n\n')

    f.write(f'pub enum LD_GRIDS {{\n')
    for arr_name in const_list:
        f.write(f'    {arr_name},\n')
    f.write(f'}}\n\n')

    f.write(f'pub fn get_LD_grid(grid: LD_GRIDS) -> &\'static [(f64, f64, f64)] {{\n')
    f.write(f'    match grid {{\n')
    for arr_name in const_list:
        f.write(f'        LD_GRIDS::{arr_name} => &LD_DATA[LD_OFFSETS["{arr_name}"].0 .. LD_OFFSETS["{arr_name}"].1],\n')
    f.write(f'    }}\n')
    f.write(f'}}')