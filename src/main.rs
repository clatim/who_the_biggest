use rand::Rng;
use nalgebra::{DMatrix};

fn main() {
    println!("Hello, world!");

    let num_unknowns: usize = 3;

    let mut rng = rand::thread_rng();
    // Generate a matrix
    let a_coefficients: Vec<f64> = (0..num_unknowns.pow(2)).map(|_| rng.gen_range(0.0..10.0)).collect();
    println!("Random vector {a_coefficients:?}");
    let m1 = DMatrix::from_vec(num_unknowns, num_unknowns, a_coefficients);
    println!("m1 = {}", m1);

    // Now generate a RHS
    let b_coeffs: Vec<f64> = (0..num_unknowns).map(|_| rng.gen_range(0.0..10.0)).collect();
    let b = DMatrix::from_vec(num_unknowns, 1, b_coeffs);
    println!("RHS will be {}", b);

    // Now invert the matrix to find the weights
    let weights: DMatrix<f64>;
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
            weights = inv * b;
            println!("The weights are {}", weights);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
    
    
}
