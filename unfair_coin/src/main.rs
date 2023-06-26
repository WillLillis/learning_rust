// surely this won't overflow
fn fact(n: u32) -> u128 {
      (1..=n as u128).product() 
}


fn calc_prob(n_trials: u32, prob: f64) -> f64 {
    let mut ret: f64 = 0.0;
    let mut tmp: f64;

    for n_heads in 1u32..=n_trials {
        ret += match n_heads % 2 {
            0 => { // if it's even
                tmp = prob.powi(n_heads as i32);
                tmp = tmp * (1.0 - prob).powi((n_trials - n_heads) as i32);
                tmp = tmp * fact(n_trials) as f64 / ( fact(n_heads) as f64 * fact(n_trials - n_heads) as f64);
                tmp
            }
            _ => 0.0, // otherwise we don't care...
        }
    }
    ret
}


fn main() {
    let num_trials: u32 = 10;
    let prob = 0.5;
    println!("The probability is {}.", calc_prob(num_trials, prob));
}
