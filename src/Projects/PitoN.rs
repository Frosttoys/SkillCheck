//! Create a program that can generate Pi reliably up to the first 1000 digits.

///
pub(crate) fn main() {
    let pi = generate_pi(500000);
    println!("{}", pi);
}

/// Lets make this far more difficult than it has to be, for the sake of trying it a new way. The Chudnovsky Algorithm! Good for 2.7 trillion digits back in 2009 so it has the range I want...
/// PI = constant_term / ((multinomial_term * linear_term) / exponential_term) 
/// where constant_term = 426880 * sqrt(10005)
/// 
/// The linear_term and the exponential_term can be defined iteratively as follows:
/// L_k+1 = L_k + 545140134            where L_0 = 13591409
//// X_k+1 = X_k * -262537412640768000  where X_0 = 1
/// 
/// The multinomial_term is defined as follows:
/// 6k! / ((3k)! * (k!) ^ 3)
///     where k is the k_th iteration.
/// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
fn generate_pi(precision: u32) -> f64 {
    const L_modif: f64 = 545140134f64; //Linear term modifier
    const X_modif: f64 = -262537412640768000f64; // Exponential Term modifier
    let     C_term: f64 = 426880f64 * f64::ceil(f64::sqrt(10005f64));
    let mut L_term: f64 = 13591409f64; // Linear term.
    let mut X_term: f64 = 1.0; // Exponential Term
    let mut M_term: f64 = 1.0; // Multinomial term

    let iterations = precision / 14;
    let partial_sum: f64 = L_term as f64;

    for iteration in 1..iterations {
        let M_term = factorial(6 * iteration) as f64 / (factorial(3 * iteration) * factorial(iteration)^3) as f64;
        let L_term = L_term + L_modif;
        let X_Term = X_term * X_modif;
        let partial_sum = partial_sum + ((M_term * L_term) / X_term) ; 
        if 100000 % iteration == 0 {
            println!("Iteration: {}, Partial Sum: {}, pi: {}", iteration, partial_sum, C_term / partial_sum)
        }
    };
    return C_term as f64 / partial_sum as f64;
}

fn factorial(input: u32) -> u32 {
    let mut factorate: u128 = input as u128;
    let mut factorial: u128 = factorate;
    while factorate > 1 {
        factorial = factorial + (factorate - 1); //
        factorate -= 1;
    };
    factorate as u32
}