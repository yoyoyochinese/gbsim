use nalgebra::{DMatrix, DVector};

pub struct GbsSimulator {
    adjacency_matrix: DMatrix<f64>,
    scaling_factor: f64,
}

impl GbsSimulator {
    pub fn new(rows: usize, colums: usize, data: Vec<f64>) -> Result<Self, String> {
        let adjacency_matrix = DMatrix::from_row_slice(rows, colums, &data);
        Self::validate(&adjacency_matrix)?;

        let scaling_factor = 1.0;

        Ok(Self {
            adjacency_matrix,
            scaling_factor,
        })
    }

    fn validate(matrix: &DMatrix<f64>) -> Result<(), String> {
        let n = matrix.nrows();
        let m = matrix.ncols();

        if n != m {
            return Err(
                "The input matrix is not square. Please check the input and try again.".to_string(),
            );
        }

        for i in 0..n {
            for j in 0..m {
                if matrix[(i, j)] != matrix[(j, i)] {
                    return Err(
                        "The input matrix is not symmetric. Please check the input and try again."
                            .to_string(),
                    );
                }
            }
        }

        Ok(())
    }
}
