use nalgebra::{Complex, DMatrix, DVector, SymmetricEigen};

// assume the input is a real symmetric matrix
// TODO: implement the general case
pub fn takagi_decomposition(
    adjacency_matrix: DMatrix<f64>,
) -> (DVector<f64>, DMatrix<Complex<f64>>) {
    let n = adjacency_matrix.nrows();
    let m = adjacency_matrix.ncols();

    assert!(n == m, "The input matrix is not square");

    println!(
        "Takagi decomposition of Matrix {:?} in progress...",
        adjacency_matrix,
    );

    let eigen = SymmetricEigen::new(adjacency_matrix);
    let eigenvalues = eigen.eigenvalues;

    let takagi_eigenvalues: DVector<f64> = eigenvalues.map(|x| x.abs());

    let signs: DVector<f64> = eigenvalues.map(|x| (-1.0f64).powf(1.0 + heaviside(x, 1.0)));
    let phases: DVector<Complex<f64>> = signs.map(|x| Complex::new(x, 0.0).sqrt());

    // let uc = eigen.eigenvectors * DMatrix::from_diagonal(&phases);
    let uc = eigen.eigenvectors.map(|x| Complex::new(x, 0.0)) * DMatrix::from_diagonal(&phases);

    let (vals_sorted, uc_sorted) = sort_indices(&takagi_eigenvalues, &uc, true);

    println!("Squeezing parameters: {:?}", vals_sorted);
    println!("Unitary matrix: {:?}", uc_sorted);

    (vals_sorted, uc_sorted)
}

fn heaviside(x1: f64, x2: f64) -> f64 {
    if x1 < 0.0 {
        0.0
    } else if x1 > 0.0 {
        1.0
    } else {
        x2
    }
}

fn sort_indices(
    vals: &DVector<f64>,
    uc: &DMatrix<Complex<f64>>,
    svd_order: bool,
) -> (DVector<f64>, DMatrix<Complex<f64>>) {
    let mut indices: Vec<usize> = (0..vals.len()).collect();
    let mut vals_sorted: DVector<f64> = DVector::zeros(vals.len());
    let mut uc_sorted: DMatrix<Complex<f64>> =
        DMatrix::from_element(uc.nrows(), uc.ncols(), Complex::new(0.0, 0.0));
    indices.sort_by(|&i, &j| vals[i].total_cmp(&vals[j]));

    if svd_order {
        indices.reverse();
    }

    for (k, &i) in indices.iter().enumerate() {
        vals_sorted[k] = vals[i];
        uc_sorted.set_column(k, &uc.column(i));
    }

    (vals_sorted, uc_sorted)
}
