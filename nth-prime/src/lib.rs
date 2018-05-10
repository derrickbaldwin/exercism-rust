
fn is_prime(n: u32) -> bool {
    let factor_range = (n as f64).sqrt() as u32 + 1;
    for i in 2..factor_range {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find(n: u32) -> u32 {
    let mut prime_counter = 1;
    let mut idx = 3;
    while prime_counter < n {
        if is_prime(idx) {
            prime_counter += 1;
        }
        idx += 2;
    }
    idx - 2 
}

pub fn nth(n: u32) -> Option<u32> {
    match n {
        n if n < 1 => None,
        1          => Some(2),
        _          => Some(find(n)) 
    }
}
