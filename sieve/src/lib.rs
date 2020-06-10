pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut range = (2..=upper_bound).collect::<Vec<u64>>();
    let mut primes = vec![];
    while (range[0] as f64) <= (upper_bound as f64).sqrt() {
        primes.push(range[0]);
        let mut next = vec![];
        for i in 1..range.len() {
            if range[i] % range[0] != 0 {
                next.push(range[i]);
            }
        }
        range = next;
    }
    primes.append(&mut range);
    primes
}

// vecにはfilterとretainという似たメソッドがある
// filterは非破壊的（memory allocationを伴う）が、retainは破壊的