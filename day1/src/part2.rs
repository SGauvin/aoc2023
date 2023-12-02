macro_rules! matches_starts_with {
    ( $b:expr, $( $x:expr, $y:expr ),* ) => {
        $(
            if $b.starts_with($x) {
                return Some($y);
            }
        )*
    };
}

#[inline]
pub fn part_two(data: &[u8]) -> u32 {
    fn get_num_from_substr(substr: &[u8]) -> Option<u32> {
        matches_starts_with!(
            substr, b"1", 1, b"2", 2, b"3", 3, b"4", 4, b"5", 5, b"6", 6, b"7", 7, b"8", 8, b"9",
            9, b"one", 1, b"two", 2, b"three", 3, b"four", 4, b"five", 5, b"six", 6, b"seven", 7,
            b"eight", 8, b"nine", 9
        );
        None
    }

    let sum = data
        .split(|c| *c == b'\n')
        .rev() // reverse & skip one to discard the last empty line
        .skip(1)
        .map(|x| {
            let first_digit = {
                let mut first_digit = 0;
                for i in 0..x.len() {
                    if let Some(num) = get_num_from_substr(&x[i..]) {
                        first_digit = num;
                        break;
                    }
                }
                first_digit
            };
            let last_digit = {
                let mut last_digit = 0;
                for i in (0..x.len()).rev() {
                    if let Some(num) = get_num_from_substr(&x[i..]) {
                        last_digit = num;
                        break;
                    }
                }
                last_digit
            };
            (first_digit * 10 + last_digit) as u32
        })
        .sum::<u32>();
    sum
}
