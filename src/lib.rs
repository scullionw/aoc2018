#[macro_export]
macro_rules! benchtest {
    ($($name:ident: $compute:expr => $ans:expr),*) => (
        #[cfg(test)]
        mod tests {
            use super::*;
            $(
                #[test]
                fn $name() {
                    assert_eq!($compute, $ans);
                }
            )*
        }

        #[cfg(test)]
        mod benches {
            use super::*;
            extern crate test;
            use self::test::Bencher;

            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    b.iter(|| $compute);
                }
            )*

        }
    )
}