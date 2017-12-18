/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert!(mat1.len() > 0);
    assert!(mat2.len() > 0);
    let m1 = mat1.len();
    let n1 = mat1[0].len();

    let m2 = mat2.len();
    let n2 = mat2[0].len();
    assert_eq!(n1, m2);

    let mut product = Vec::new();
    for _ in 0..m1 { product.push(Vec::with_capacity(n2)) }

    for r in 0..m1 {
        for c in 0..n2 {
            product[r].push(0.);
            for i in 0..n1 {
                product[r][c] += mat1[r][i] * mat2[i][c]
            }  
        } 
    }
    product
}
