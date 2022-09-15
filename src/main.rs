use rand::Rng;
use nalgebra::{DMatrix,DVector};


enum InversionError {
    SingularMatrix,
}

fn main() {
    let difficulty = 2;
    let num_unknowns: usize = difficulty;

    match generate_solution(num_unknowns) {
        Ok(weight) => println!("Max weight and index is {weight:?}"),
        Err(_) => println!("Matrix was singular. Try again!"),
    }
    // TODO let user guess the position based on the system.
}

fn generate_solution(num_unknowns: usize) -> Result<(usize,f64), InversionError> {
    let mut rng = rand::thread_rng();
    // Generate a matrix
    let a_coefficients: Vec<f64> = (0..num_unknowns.pow(2)).map(|_| rng.gen_range(0.0..10.0)).collect();
    let m1 = DMatrix::from_vec(num_unknowns, num_unknowns, a_coefficients);

    // Now generate a RHS
    let b_coeffs: Vec<f64> = (0..num_unknowns).map(|_| rng.gen_range(0.0..10.0)).collect();
    let b = DVector::from_vec(b_coeffs);
    // println!("RHS will be {}", b);
    println!("The system is {m1} = {b}");

    // Now invert the matrix to find the weights
    match m1.try_inverse() {
        Some(inv) => {
            let weights = inv * b;
            println!("The weights are {}", weights);
            println!("the positon of max is {:?}", weights.argmax());
            Ok(weights.argmax())
        }
        None => {
            Err(InversionError::SingularMatrix)
        }
    }
}
