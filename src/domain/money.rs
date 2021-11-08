use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign, Neg, Sub, SubAssign,
)]

pub struct Money(pub i64);

impl Money {
    pub fn is_positive_or_zero(&self) -> bool {
        self.0 >= 0
    }

    pub fn is_negative(&self) -> bool {
        !self.is_positive_or_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_add() {
        // Given
        let m = Money(10);
        let n = Money(5);

        // Expect
        assert_eq!(m + n, Money(15));
        assert_eq!(n + m, Money(15));
    }

    #[test]
    fn test_money_addassign() {
        // Given
        let mut m = Money(10);
        let n = Money(5);
        // When
        m += n;
        // Expect
        assert_eq!(m, Money(15));
    }

    #[test]
    fn test_is_cmp() {
        // Given
        let m = Money(10);
        let n = Money(5);

        // Expect
        assert!(m > n);
        assert!(n < m);
        assert!(m >= m);
        assert!(m == m);
        assert!(m != n);
    }
}
