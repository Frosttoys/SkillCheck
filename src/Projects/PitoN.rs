//! Create a program that can generate Pi reliably up to the first 1000 digits.
use factorial::Factorial as factor;

pub(crate) fn main() {
    let pi = generate_pi(10000);
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
fn generate_pi(precision: u32) -> String {
    const L_modif: i64 = 545140134; //Linear term modifier
    const X_modif: i64 = -262537412640768000; // Exponential Term modifier
    let     C_term: i64 = 426880 * f64::ceil(f64::sqrt(10005f64)) as i64;
    let mut L_term: i64 = 13591409; // Linear term.
    let mut X_term: i64 = 1; // Exponential Term
    let mut M_term: i64 = 0; // Multinomial term

    let iterations = precision / 14;
    let partial_sum: i64 = L_term ;

    for iteration in 1..iterations {
        let M_term = (factor::factorial(&(6 * iteration)) / 
                     (factor::factorial(&(3 * iteration)) * 
                      factor::factorial(&(iteration^3)))) as i64; //Overflows after 2 iterations...
        let L_term = L_term + L_modif;
        let X_Term = X_term * X_modif;
        let partial_sum = partial_sum + ((M_term * L_term) / X_term);
        if 100 % iteration == 0 {
            println!("Iteration: {}, Partial Sum: {}, pi: {}", iteration, partial_sum, C_term / partial_sum)
        }
    };
    //C_term  / partial_sum;
    return String::from("");
}
