/// Find the first factor (other than 1) of a number
fn first_factor(x: u64) -> u64 {
  if x % 2 == 0 { return 2; };
  for n in (3..).step_by(2).take_while(|m| m*m <= x) {
      if x % n == 0 { return n; };
  }
  return x;
}

/// Test whether a number is prime. Checks every odd number up to sqrt(n).
pub fn is_prime(n : u64) -> bool {
  if n <= 1 { return false; }
  return first_factor(n) == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_prime_0() {
        assert_eq!(is_prime(0), false);
    }

    #[test]
    fn test_is_not_prime_1() {
      assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_is_prime_2() {
      assert_eq!(is_prime(2), true);
    }

    #[test]
    fn test_is_prime_3() {
      assert_eq!(is_prime(3), true);
    }

    #[test]
    fn test_is_not_prime_4() {
      assert_eq!(is_prime(4), false);
    }

    #[test]
    fn test_is_prime_5() {
      assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_not_prime_6() {
      assert_eq!(is_prime(6), false);
    }

    #[test]
    fn test_is_prime_7() {
      assert_eq!(is_prime(7), true);
    }

    #[test]
    fn test_is_not_prime_15() {
      assert_eq!(is_prime(15), false);
    }

    #[test]
    fn test_is_not_prime_17() {
      assert_eq!(is_prime(17), true);
    }

    #[test]
    fn test_is_not_prime_49() {
      assert_eq!(is_prime(49), false);
    }

    #[test]
    fn test_is_prime_53() {
      assert_eq!(is_prime(53), true);
    }

    #[test]
    fn test_is_prime_big() {
      assert_eq!(is_prime(67280421310721), true);
    }
}