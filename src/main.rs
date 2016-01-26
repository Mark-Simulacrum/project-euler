#![feature(test)]

extern crate test;

extern crate primal;

#[allow(dead_code)]
mod primes {
    fn is_prime(num: u64) -> bool {
        for i in 2..(num / 2 + 1) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    pub struct Prime {
        pub curr: u64
    }

    impl Iterator for Prime {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let mut next = self.curr + 1;
            while !is_prime(next) {
                next += 1;
            }

            self.curr = next;

            Some(self.curr)
        }
    }

    pub fn primes() -> Prime {
        Prime { curr: 0 }
    }
}

#[allow(dead_code)]
pub mod problem1 {
    pub fn main() -> u32 {
        (0..1000)
            .filter(|num| num % 5 == 0 || num % 3 == 0)
            .fold(0, |sum, value| sum + value)
    }
}

#[allow(dead_code)]
pub mod problem2 {
    struct Fibonacci {
        curr: u64,
        next: u64
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }

    pub fn main() -> u64 {
        fibonacci()
            .take_while(|&x| x <= 4_000_000)
            .filter(|&x| x % 2 == 0)
            .fold(0, |sum, x| sum + x)
    }
}

#[allow(dead_code)]
pub mod problem3 {
    use super::primes::Prime;

    pub fn main() -> u64 {
        let mut num = 600851475143;
        let mut highest_prime_factor = 0;

        let prime = Prime { curr: 1 };

        for i in prime {
            if num % i == 0 {
                highest_prime_factor = i;

                while num % i == 0 && num >= 2 {
                    num /= i;
                }
            }


            if num == 1 {
                break;
            }
        }

        highest_prime_factor
    }
}

#[allow(dead_code)]
pub mod problem4 {
    use std::cmp;

    const FIRST_3_DIGIT: u64 = 100;
    const FIRST_4_DIGIT: u64 = 1_000;

    fn is_palindrome(num: u64) -> bool {
        let s = num.to_string();
        s.chars().eq(s.chars().rev())
    }

    pub fn main() -> u64 {
        let mut largest = 0;
        for i in (FIRST_3_DIGIT..FIRST_4_DIGIT).rev() {
            let start = cmp::max(FIRST_3_DIGIT, largest / i + 1);

            for j in (start..FIRST_4_DIGIT).rev() {
                if is_palindrome(i * j) {
                    largest = i * j;
                    break;
                }
            }
        }
        largest
    }
}

#[allow(dead_code)]
pub mod problem5 {
    fn get_smallest() -> Option<u64> {
        let max = (11..20).fold(1, |product, n| product * n);
        for i in 1..max {
            if i % 11 == 0 &&
                i % 12 == 0 &&
                i % 13 == 0 &&
                i % 14 == 0 &&
                i % 15 == 0 &&
                i % 16 == 0 &&
                i % 17 == 0 &&
                i % 18 == 0 &&
                i % 19 == 0 &&
                i % 20 == 0 {
                    return Some(i);
            }
        }
        None
    }

    pub fn main() -> u64 {
        get_smallest().unwrap()
    }
}

#[allow(dead_code)]
pub mod problem6 {
    pub fn main() -> u64 {
        let mut sum_of_squares:u64 = 0;
        let mut square_of_sums:u64 = 0;
        for i in 1..101u64 {
            sum_of_squares += i.pow(2);
            square_of_sums += i;
        };
        square_of_sums = square_of_sums.pow(2);

        square_of_sums - sum_of_squares
    }
}

#[allow(dead_code)]
pub mod problem7 {
    use super::primes::primes;

    pub fn main() -> u64 {
        primes().nth(10_001).unwrap()
    }
}

#[allow(dead_code)]
pub mod problem8 {
    pub fn main() -> u64 {
        let input =
                    "73167176531330624919225119674426574742355349194934\
                    96983520312774506326239578318016984801869478851843\
                    85861560789112949495459501737958331952853208805511\
                    12540698747158523863050715693290963295227443043557\
                    66896648950445244523161731856403098711121722383113\
                    62229893423380308135336276614282806444486645238749\
                    30358907296290491560440772390713810515859307960866\
                    70172427121883998797908792274921901699720888093776\
                    65727333001053367881220235421809751254540594752243\
                    52584907711670556013604839586446706324415722155397\
                    53697817977846174064955149290862569321978468622482\
                    83972241375657056057490261407972968652414535100474\
                    82166370484403199890008895243450658541227588666881\
                    16427171479924442928230863465674813919123162824586\
                    17866458359124566529476545682848912883142607690042\
                    24219022671055626321111109370544217506941658960408\
                    07198403850962455444362981230987879927244284909188\
                    84580156166097919133875499200524063689912560717606\
                    05886116467109405077541002256983155200055935729725\
                    71636269561882670428252483600823257530420752963450";

        let input_bytes = input.as_bytes();
        let span_width = 13;

        let mut largest = 0;
        for i in 0..(input_bytes.len() - span_width + 1) {
            let mut product = 1u64;
            for j in 0..span_width {
                product *= (input_bytes[i + j] - 48) as u64;
            }
            if product > largest {
                largest = product;
                // println!("{:?}", str::from_utf8(&input_bytes[i..i + span_width]).unwrap())
            }
        };

        largest
    }
}

#[allow(dead_code)]
pub mod problem9 {
    pub fn main() -> u64 {
        let s = 1000u64;

        for a in 1..1000u64 {
            for b in 1..1000u64 - a {
                let c = s - a - b;
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    return a * b * (1000 - a - b)
                }
            }
        }
        0
    }
}

#[allow(dead_code)]
pub mod problem10 {
    pub fn main() -> usize {
        let mut sum = 0;
        const SIZE: usize = 2_000_000;
        let mut slots = [true; SIZE];
        slots[0] = false;
        slots[1] = false;

        for stride in 2..(SIZE/2) {
            let mut pos = stride;
            while pos < (SIZE - stride) {
                pos += stride;
                slots[pos] = false;
            }
        }

        for (idx, &pr) in slots.into_iter().enumerate() {
            if pr {
                sum += idx;
            }
        }

        sum
    }
}

#[allow(dead_code)]
pub mod problem11 {
    use std::cmp;

    pub fn main() -> u32 {
        let input = vec!(
            vec!(08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08),
            vec!(49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00),
            vec!(81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65),
            vec!(52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91),
            vec!(22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80),
            vec!(24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50),
            vec!(32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70),
            vec!(67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21),
            vec!(24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72),
            vec!(21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95),
            vec!(78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92),
            vec!(16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57),
            vec!(86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58),
            vec!(19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40),
            vec!(04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66),
            vec!(88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69),
            vec!(04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36),
            vec!(20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16),
            vec!(20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54),
            vec!(01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48)
        );

        let mut largest_product = 0;

        for row_index in 0..input.len() {
            let row_length = input[row_index].len();
            for column_index in 0..row_length {
                // Down
                if row_index + 3 < input.len() {
                    let product =
                        input[row_index][column_index] *
                        input[row_index + 1][column_index] *
                        input[row_index + 2][column_index] *
                        input[row_index + 3][column_index];


                    largest_product = cmp::max(largest_product, product);
                }

                // Diagonally right
                if row_index + 3 < input.len() && column_index + 3 < row_length {
                    let product =
                        input[row_index][column_index] *
                        input[row_index + 1][column_index + 1] *
                        input[row_index + 2][column_index + 2] *
                        input[row_index + 3][column_index + 3];


                    largest_product = cmp::max(largest_product, product);
                }

                // Diagonally left
                if row_index + 3 < input.len() && column_index > 3 {
                    let product =
                        input[row_index][column_index] *
                        input[row_index + 1][column_index - 1] *
                        input[row_index + 2][column_index - 2] *
                        input[row_index + 3][column_index - 3];


                    largest_product = cmp::max(largest_product, product);
                }

                // Sideways
                if column_index + 3 < row_length {
                    let product =
                        input[row_index][column_index] *
                        input[row_index][column_index + 1] *
                        input[row_index][column_index + 2] *
                        input[row_index][column_index + 3];


                    largest_product = cmp::max(largest_product, product);
                }

            }
        }

        return largest_product;
    }
}

#[allow(dead_code)]
pub mod problem12 {
    use primal::*;
    use std::cmp::max;

    fn num_divisors(n: u64) -> Option<usize> {
        let (_lo, hi) = estimate_nth_prime(n);
        let sieve = Sieve::new(hi as usize);

        match sieve.factor(n as usize) {
            Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
            Err(_) => None,
        }
    }

    fn sum_num_up_to(n: u64) -> u64 {
        let mut sum = 0;
        for num in 1..n {
            sum += num
        }
        sum
    }

    pub fn main() -> u64 {
        let mut current = 1;
        let mut max_factor_seen = 0;

        while max_factor_seen <= 500 {
            let factor_amount = num_divisors(current).unwrap();
            max_factor_seen = max(max_factor_seen, factor_amount);

            current += (9u64).pow(10);
            println!("Continuing: {}, factors: {}, max_factors: {}", current, factor_amount, max_factor_seen);
        }

        println!("Computed upper_bound: {}", current);

        let upper_bound = current;
        max_factor_seen = 0;
        for num in (1..upper_bound).rev() {
            let current = sum_num_up_to(num);

            let factor_amount = num_divisors(current).unwrap();
            max_factor_seen = max(max_factor_seen, factor_amount);

            println!("current: {}: {} ; max: {}", current, factor_amount, max_factor_seen);

            if factor_amount > 500 {
                return current;
            }
        }

        0
    }
}

#[allow(dead_code)]
pub mod problem_x {
    pub fn main() -> u64 {
        1
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| problem2::main());
    }

    // #[bench]
    // fn bench_2(b: &mut Bencher) {
    //     b.iter(|| problem2::main2());
    // }
}

fn main() {
    // println!("Problem #1: {}", problem1::main());
    println!("Problem #2: {}", problem2::main());
    // println!("Problem #3: {}", problem3::main());
    // println!("Problem #4: {}", problem4::main());
    // println!("Problem #4: {}", problem4::main2());
    // println!("Problem #5: {}", problem5::main());
    // println!("Problem #6: {}", problem6::main());
    // println!("Problem #7: {}", problem7::main());
    // println!("Problem #8: {}", problem8::main());
    // println!("Problem #9: {}", problem9::main());
    // println!("Problem #10: {}", problem10::main());
    // println!("Problem #11: {:?}", problem11::main());
    // println!("Problem #12: {:?}", problem12::main());
}
