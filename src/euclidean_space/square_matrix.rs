use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct SquareMatrix<const N: usize> {
    matrix: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub fn zero() -> Self {
        return SquareMatrix {
            matrix: [[0.0; N]; N],
        };
    }

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

    pub const fn new(values: [[Float; N]; N]) -> SquareMatrix<N> {
        return SquareMatrix { matrix: values };
    }

    pub fn from_vec(values: Vec<Float>) -> SquareMatrix<N> {
        // TODO: rename to from_array
        if values.len() != N * N {
            panic!("size of vector and size of matrix not matched");
        }

        let mut data = [[0.0; N]; N];

        for y in 0..N {
            for x in 0..N {
                data[y][x] = values[y * N + x];
            }
        }

        return SquareMatrix { matrix: data };
    }

    pub fn from_diag(diag: [Float; N]) -> Self {
        let mut matrix = Self::zero();
        for i in 0..N {
            matrix[i][i] = diag[i];
        }

        return matrix;
    }

    pub fn is_identity(&self) -> bool {
        for y in 0..N {
            for x in 0..N {
                if x == y {
                    if self[y][x] != 1.0 {
                        return false;
                    }
                } else {
                    if self[y][x] != 0.0 {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    pub fn transpose(&self) -> SquareMatrix<N> {
        let mut transposed = SquareMatrix::nan();

        for y in 0..N {
            for x in 0..N {
                transposed[y][x] = self[x][y];
            }
        }

        return transposed;
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

    pub fn display(&self) {
        for y in 0..N {
            for x in 0..N {
                print!("{:.4} ", self.matrix[y][x]);
            }
            println!();
        }
    }
}

impl<const N: usize> Index<usize> for SquareMatrix<N> {
    type Output = [Float; N];
    fn index(&self, idx: usize) -> &[Float; N] {
        return &self.matrix[idx];
    }
}

impl<const N: usize> IndexMut<usize> for SquareMatrix<N> {
    fn index_mut(&mut self, idx: usize) -> &mut [Float; N] {
        return &mut self.matrix[idx];
    }
}

impl Mul<RGB> for SquareMatrix<3> {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        let array = [rhs.r, rhs.g, rhs.b];
        return RGB {
            r: inner_product(&self[0], &array).value,
            g: inner_product(&self[1], &array).value,
            b: inner_product(&self[2], &array).value,
        };
    }
}

impl Mul<CIEXYZ> for SquareMatrix<3> {
    type Output = CIEXYZ;

    fn mul(self, rhs: CIEXYZ) -> Self::Output {
        let array = [rhs.x, rhs.y, rhs.z];

        return CIEXYZ {
            x: inner_product(&self[0], &array).value,
            y: inner_product(&self[1], &array).value,
            z: inner_product(&self[2], &array).value,
        };
    }
}

impl Mul<SquareMatrix<3>> for SquareMatrix<3> {
    type Output = SquareMatrix<3>;

    fn mul(self, rhs: SquareMatrix<3>) -> Self::Output {
        let mut product = SquareMatrix::<3>::zero();
        for x in 0..3 {
            for y in 0..3 {
                let compensated_float = inner_product(
                    &[self[x][0], self[x][1], self[x][2]],
                    &[rhs[0][y], rhs[1][y], rhs[2][y]],
                );
                product[x][y] = compensated_float.eval();
            }
        }

        return product;
    }
}

impl Mul<SquareMatrix<4>> for SquareMatrix<4> {
    type Output = SquareMatrix<4>;

    fn mul(self, rhs: SquareMatrix<4>) -> Self::Output {
        let mut product = SquareMatrix::<4>::zero();
        for x in 0..4 {
            for y in 0..4 {
                let compensated_float = inner_product(
                    &[self[x][0], self[x][1], self[x][2], self[x][3]],
                    &[rhs[0][y], rhs[1][y], rhs[2][y], rhs[3][y]],
                );
                product[x][y] = compensated_float.eval();
            }
        }

        return product;
    }
}

impl<const N: usize> Display for SquareMatrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..N {
            for x in 0..N {
                write!(f, "{:.4} ", self[y][x]).expect("error");
            }
            write!(f, "\n").expect("error");
        }
        Ok(())
    }
}
