// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32,) -> u32 {
    let mut output = 1;
    let mut i = 1;
    while i <= n {
        output = output * i;
        i = i + 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
