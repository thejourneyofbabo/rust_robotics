/// 두 숫자의 합을 반환
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(-1, 1), 0);
    }
}
