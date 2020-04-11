// pub fn is_leap_year(year: u64) -> bool {
//     if year % 4 == 0 {
//         !(year % 100 == 0 && year % 400 != 0)
//     } else {
//         false
//     }
// }

pub fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
