use rand::Rng;
use nalgebra::{DMatrix,DVector};

fn main() {
    println!("Hello, world!");

    let difficulty = 2;
    let num_unknowns: usize = difficulty;

    let answer: f64 = generate_solution(num_unknowns);
    // TODO let user guess the position based on the system.
}

fn generate_solution(num_unknowns: usize) -> f64 {
    let mut rng = rand::thread_rng();
    // Generate a matrix
    let a_coefficients: Vec<f64> = (0..num_unknowns.pow(2)).map(|_| rng.gen_range(0.0..10.0)).collect();
    let m1 = DMatrix::from_vec(num_unknowns, num_unknowns, a_coefficients);

    // Now generate a RHS
    let b_coeffs: Vec<f64> = (0..num_unknowns).map(|_| rng.gen_range(0.0..10.0)).collect();
    let b = DMatrix::from_vec(num_unknowns, 1, b_coeffs);
    // println!("RHS will be {}", b);
    println!("The system is {m1} = {b}");

    // Now invert the matrix to find the weights
    let max_weight: f64;
    match m1.try_inverse() {
        Some(inv) => {
            let weights: DMatrix<f64> = inv * b;
            println!("The weights are {}", weights);
            max_weight = weights.max();            
        }
        None => {
            println!("m1 is not invertible!");
            max_weight = 0.0;
        }
    }
    // TODO return position of maximum weight
    return max_weight;
}
