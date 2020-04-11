use std::vec::Vec;
pub fn nth(n: u32) -> u32 {
    
    let mut i: u32 = 3;
    let mut primes = Vec::with_capacity(n as usize);
    primes.push(2);
    loop {
        if primes.len() >= (n as usize)+1 {
            break
        }
        let mut flg = true;
        for j in primes.iter() {
            if i % j == 0 {
                flg = false;
                break
            }
        }
        if flg {
            primes.push(i);
        }
        i += 1;
    }
    primes[(n as usize)]
}
