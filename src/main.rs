use rand::Rng;
use nalgebra::{DMatrix,DVector};
use std::io;

enum InversionError {
    SingularMatrix,
}

fn main() {
    let mut difficulty = 1;
    let mut failed_attempts = 0;

    loop {
        let correct = user_guess(difficulty);
        if correct {
            difficulty += 1;
            failed_attempts = 0;
            println!("You win! Increasing level to {difficulty}");
        } else {
            failed_attempts += 1;
            println!("This is failed attempt {failed_attempts}. Try again");
            if failed_attempts == 2 {
                println!("Last attempt before you game over!");
            } else if failed_attempts == 3 {
                println!("GAME OVER!");
                break;
            }

        }
    }

}


fn user_guess(difficulty: usize) -> bool {
    let num_unknowns: usize = difficulty;
    let position: usize;
    let _weight: f64;
    loop {
        match generate_solution(num_unknowns) {
            Ok(answer) => {
                (position, _weight) = answer;
                break;
            },
            Err(_) => {},
        }
    }
    
    println!("Please guess the largest solution in the system:");
    let mut guess: usize;
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line!");
        guess = inp
            .trim()
            .parse()
            .expect("Input not an integer!");

        if guess < num_unknowns {
            break;
        } else {
            println!("Input must be in [0,{}]", num_unknowns-1);
        }
    }

    if guess == position {
        true
    } else {
        false
    }

}


fn generate_solution(num_unknowns: usize) -> Result<(usize,f64), InversionError> {
    let mut rng = rand::thread_rng();
    // Generate a matrix
    let a_coefficients: Vec<f64> = (0..num_unknowns.pow(2)).map(|_| rng.gen_range(0.0..10.0)).collect();
    let m1 = DMatrix::from_vec(num_unknowns, num_unknowns, a_coefficients);

    // Now generate a RHS
    let b_coeffs: Vec<f64> = (0..num_unknowns).map(|_| rng.gen_range(0.0..10.0)).collect();
    let b = DVector::from_vec(b_coeffs);
    // Generate vector of unknowns for print
    let variables: Vec<String> = (0..num_unknowns).map(|n| format!("x{}", n)).collect();
    let answers = DVector::from_vec(variables);
    println!("The system is {m1}{answers} = {b}");


    // Now invert the matrix to find the weights
    match m1.try_inverse() {
        Some(inv) => {
            let weights = inv * b;
            // println!("The weights are {}", weights);
            // println!("the positon of max is {:?}", weights.argmax());
            Ok(weights.argmax())
        }
        None => {
            Err(InversionError::SingularMatrix)
        }
    }
}

#[cfg(test)]
mod my_first_test_module {

    #[test]
    fn my_first_test() {
        assert!(1+1 == 2);
    }

    #[test]
    #[should_panic]
    fn my_second_test() {
        assert!(1+1 == 3);
    }

}