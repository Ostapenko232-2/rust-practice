fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let mut n = n % len;
    if n < 0 {
        n += len;
    }

    let n = n as usize;
    format!("{}{}", &s[s.len()-n..], &s[..s.len()-n])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate2(s: &str, n: &isize) -> String {
        rotate(s.to_string(), *n)
    }

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate2(s, n), exp.to_string());
        });
    }
}
