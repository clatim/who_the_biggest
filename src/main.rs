use rand::Rng;
use nalgebra::{DMatrix,DVector};
use std::io;

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

    let (position, _weight) = generate_solution(num_unknowns)
        .expect("Solution couldn't be generated. Exiting.");
    
    let mut guess: usize;
    loop {
        println!("Please guess the largest variable in the system:");
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
            println!("Guess must be between 0 and {}.", num_unknowns-1);
        }
    }

    if guess == position {
        true
    } else {
        false
    }

}


fn generate_solution(num_unknowns: usize) -> Option<(usize,f64)> {
    let mut rng = rand::thread_rng();
    // Generate a matrix
    let m1 = DMatrix::from_vec(num_unknowns, num_unknowns, 
        (0..num_unknowns.pow(2)).map(|_| rng.gen_range(0.0..10.0)).collect()
    );
    // Now generate a RHS
    let b = DVector::from_vec(
        (0..num_unknowns).map(|_| rng.gen_range(0.0..10.0)).collect()
    );
    // Generate vector of unknowns for print
    let variables = DVector::from_vec(
        (0..num_unknowns).map(|n| format!("x{}", n)).collect()
    );

    println!("The system is {m1}{variables} = {b}");

    // Now invert the matrix to find the weights
    let inv = m1.try_inverse()?;
    let weights = inv * b;
    Some(weights.argmax())
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