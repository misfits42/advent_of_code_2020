//
// Chinese Remainder Theorem (CRT) solving functions humbly borrowed from Rosetta Code.
// 
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
// 

/// Solver for Chinese remainder theorem (CRT) using the provided modulii and residues.
pub fn solve_crt(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    } 
    Some(sum % prod)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
