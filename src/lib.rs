#[macro_export]
macro_rules! test {
    ($a:expr, $b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn part_a_test() {
                assert_eq!(solve_a(INPUT), $a);
            }
            #[test]
            fn part_b_test() {
                assert_eq!(solve_b(INPUT), $b);
            }
        }
    };
}

#[macro_export]
macro_rules! bench {
    () => {
        #[cfg(test)]
        mod bench {
            use super::*;
            extern crate test;
            use self::test::Bencher;

            #[bench]
            fn part_a_bench(b: &mut Bencher) {
                b.iter(|| solve_a(INPUT));
            }

            #[bench]
            fn part_b_bench(b: &mut Bencher) {
                b.iter(|| solve_b(INPUT));
            }
        }
    };
}
