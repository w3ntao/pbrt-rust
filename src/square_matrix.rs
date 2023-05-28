pub struct SquareMatrix<const LENGTH: usize> {
    matrix: [[f32; LENGTH]; LENGTH],
}

impl<const LENGTH: usize> Default for SquareMatrix<LENGTH> {
    fn default() -> Self {
        return SquareMatrix {
            matrix: [[0.0; LENGTH]; LENGTH],
        };
    }
}

impl<const LENGTH: usize> SquareMatrix<LENGTH> {
    fn identity() -> Self {
        let mut data = [[0.0; LENGTH]; LENGTH];
        for idx in 0..LENGTH {
            data[idx][idx] = 1.0;
        }

        return SquareMatrix { matrix: data };
    }
}
