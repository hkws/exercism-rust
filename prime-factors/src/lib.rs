pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    //let mut primes: Vec<u64> = vec![]
    let mut trynum: u64 = 2;
    let mut num: u64 = n;
    while num != 1u64 {
        // ここは不要
        // trynumは単調増加なので、素数でないtrynumは
        // その因数で必ずそれ以前に割り算されているはず
        if !is_prime(trynum) {
            trynum += 1;
            continue;
        }
        while num % trynum == 0u64 {
            num /= trynum;
            factors.push(trynum)
        }
        if trynum == 2u64 {
            trynum = 3u64;
        } else {
            trynum += 2u64;
        }
    }
    factors
}

pub fn is_prime(n: u64) -> bool {
    match n {
        2 | 3 => true,
        n if n < 2 || n % 2 == 0 => false,
        n if n < 9 => true,
        n if n % 3 == 0 => false,
        _ => {
            let limit = (n as f64).powf(0.5) as u64;
            let mut factor = 5;
            while factor <= limit {
                if n % factor == 0 || n % (factor + 2) == 0 {
                    return false;
                }
                factor += 6;
            }
            true
        }
    }
}

// 模範解答
// pub fn factors(mut n: u64) -> Vec<u64> {
//     let mut factors = Vec::new();
//     let mut candidates = 2..;

//     while n > 1 {
//         let x = candidates.next().unwrap();

//         while n % x == 0 {
//             n /= x;
//             factors.push(x);
//         }
//     }

//     factors
// }
