use nalgebra::{DMatrix, DVector, SymmetricEigen};

// assume the input is a real symmetric matrix
// TODO: implement the general case
pub fn takagi_decomposition(adjacency_matrix: DMatrix<f64>) {
    let n = adjacency_matrix.nrows();
    let m = adjacency_matrix.ncols();

    assert!(n == m, "The input matrix is not square");

    println!(
        "Takagi decomposition of Matrix {:?} in progress...",
        adjacency_matrix,
    );

    let eigen = SymmetricEigen::new(adjacency_matrix);

    let takagi_eigenvalues = eigen.eigenvalues.map(|x| x.abs());

    let signs = takagi_eigenvalues.map(|x| (-1.0f64).powf(1.0 + heaviside(x)));
    let phases = signs.map(|x| x.sqrt());

    let uc = eigen.eigenvectors * DMatrix::from_diagonal(&phases);

    let (vals_sorted, uc_sorted) = sort_indices(&takagi_eigenvalues, &uc, true);

    println!("Squeezing parameters: {:?}", vals_sorted);
    println!("Unitary matrix: {:?}", uc_sorted);
}

fn heaviside(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 0.0 {
        1.0
    } else {
        0.5
    }
}

fn sort_indices(
    vals: &DVector<f64>,
    uc: &DMatrix<f64>,
    svd_order: bool,
) -> (DVector<f64>, DMatrix<f64>) {
    let mut indices: Vec<usize> = (0..vals.len()).collect();
    let mut vals_sorted = DVector::zeros(vals.len());
    let mut uc_sorted = DMatrix::zeros(uc.nrows(), uc.ncols());
    indices.sort_by(|&i, &j| vals[i].partial_cmp(&vals[j]).unwrap());

    if svd_order {
        indices.reverse();
    }

    for (k, &i) in indices.iter().enumerate() {
        vals_sorted[k] = vals[i];
        uc_sorted.set_column(k, &uc.column(i));
    }

    (vals_sorted, uc_sorted)
}
