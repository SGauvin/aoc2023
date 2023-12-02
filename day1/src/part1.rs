#[inline]
pub fn part_one(data: &[u8]) -> u32 {
    let sum = data
        .split(|c| *c == b'\n')
        .rev() // reverse & skip one to discard the last empty line
        .skip(1)
        .map(|x| {
            let first_digit = x.iter().find(|x| x.is_ascii_digit()).unwrap() - b'0';
            let last_digit = x.iter().rfind(|x| x.is_ascii_digit()).unwrap() - b'0';
            (first_digit * 10 + last_digit) as u32
        })
        .sum::<u32>();
    sum
}
