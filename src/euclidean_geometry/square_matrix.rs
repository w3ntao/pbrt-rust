use crate::pbrt::*;
use std::mem::swap;

pub struct SquareMatrix<const N: usize> {
    matrix: [[Float; N]; N],
}

impl<const N: usize> Default for SquareMatrix<N> {
    fn default() -> Self {
        return SquareMatrix {
            matrix: [[0.0; N]; N],
        };
    }
}

impl<const N: usize> SquareMatrix<N> {
    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];
        for idx in 0..N {
            data[idx][idx] = 1.0;
        }

        return SquareMatrix { matrix: data };
    }

    pub fn nan() -> Self {
        return SquareMatrix {
            matrix: [[Float::NAN; N]; N],
        };
    }

    pub fn inverse(&self) -> SquareMatrix<N> {
        let mut indxc = [0; N];
        let mut indxr = [0; N];
        let mut ipiv = [0; N];

        let mut minv = self.matrix.clone();

        for i in 0..N {
            let mut irow = 0;
            let mut icol = 0;
            let mut big = 0.0;
            for j in 0..N {
                if ipiv[j] != 1 {
                    for k in 0..N {
                        if ipiv[k] == 0 {
                            if minv[j][k].abs() >= big {
                                big = minv[j][k].abs();
                                irow = j;
                                icol = k;
                            }
                        } else if ipiv[k] > 1 {
                            panic!("This is a singular matrix.")
                        }
                    }
                }
            }

            ipiv[icol] += 1;
            // Swap rows _irow_ and _icol_ for pivot
            if irow != icol {
                for k in 0..N {
                    (minv[irow][k], minv[icol][k]) = (minv[icol][k], minv[irow][k]);
                    //swap(&mut minv[irow][k], &mut minv[icol][k]);
                }
            }

            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol][icol] == 0.0 {
                panic!("This is a singular matrix.")
            }

            // Set $m[icol][icol]$ to one by scaling row _icol_ appropriately
            let pivinv = 1.0 / minv[icol][icol];
            minv[icol][icol] = 1.0;
            for j in 0..N {
                minv[icol][j] *= pivinv;
            }

            // Subtract this row from others to zero out their columns
            for j in 0..N {
                if j != icol {
                    let save = minv[j][icol];
                    minv[j][icol] = 0.0;
                    for k in 0..N {
                        minv[j][k] = fma(-minv[icol][k], save, minv[j][k]);
                    }
                }
            }
        }

        // Swap columns to reflect permutation
        // TODO: verify this loop
        for j in (0..N).rev() {
            if indxr[j] == indxc[j] {
                continue;
            }
            for k in 0..N {
                minv[k].swap(indxr[j], indxc[j]);
                //swap(&mut minv[k][indxr[j]], &mut minv[k][indxc[j]]);
            }
        }

        return SquareMatrix::<N> { matrix: minv };
    }
}

impl<const N: usize> ops::Index<usize> for SquareMatrix<N> {
    type Output = [f32; N];
    fn index(&self, idx: usize) -> &[f32; N] {
        return &self.matrix[idx];
    }
}

impl<const N: usize> ops::IndexMut<usize> for SquareMatrix<N> {
    fn index_mut(&mut self, idx: usize) -> &mut [f32; N] {
        return &mut self.matrix[idx];
    }
}
