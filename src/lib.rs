//! # assert_hex
//! display expression using `{:#x?}` format when false assertion causes `panic!()`.
//!
//! # why
//! Writing and testing protocol level libraries requires many tests to be written
//! with respect to byte level protocol sections in hex. This library simplifies the process
//! of viewing the differences between these types when tests fail by displaying by using the
//! `{:#x?}` representation.
//!
//! # usage
//! Replace `assert_eq` or `assert_ne` with `assert_eq_hex` or `assert_ne_hex`
//! respectively.

/// Asserts that two expressions are equal to each other
///
/// On panic, this macro will print values of the expressions in their
/// `{:#x?}` (hexadecimal) representation
#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:#x?}`,
 right: `{:#x?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  left: `{:#x?}`,
 right: `{:#x?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

/// Asserts that two expressions are not equal to each other
///
/// On panic, this macro will print values of the expressions in their
/// `{:#x?}` (hexadecimal) representation
#[macro_export]
macro_rules! assert_ne_hex {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left != right)`
  left: `{:#x?}`,
 right: `{:#x?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left != right)`
  left: `{:#x?}`,
 right: `{:#x?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_hex, assert_ne_hex};

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`
  left: `0x50`,
 right: `0x46`"#)]
    fn test_eq_0() {
        assert_eq_hex!(0x50, 0x46);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`
  left: `0xff`,
 right: `0x46`"#)]
    fn test_eq_1() {
        assert_eq_hex!(0xff, 0x46);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`
  left: `0xff`,
 right: `0x46`"#)]
    fn test_eq_2() {
        assert_eq_hex!(0xff, 0x46);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`
  left: `[
    0x0,
    0x1,
    0x2,
]`,
 right: `[
    0x46,
    0x50,
    0x40,
]`"#)]
    fn test_eq_3() {
        assert_eq_hex!(vec![0x00, 0x01, 0x02], vec![0x46, 0x50, 0x40]);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left != right)`
  left: `0x50`,
 right: `0x50`"#)]
    fn test_ne_0() {
        assert_ne_hex!(0x50, 0x50);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left != right)`
  left: `0xff`,
 right: `0xff`"#)]
    fn test_ne_1() {
        assert_ne_hex!(0xff, 0xff);
    }
}
