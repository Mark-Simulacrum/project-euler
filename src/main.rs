#![feature(test)]
#![feature(plugin)]
#![feature(inclusive_range_syntax)]
#![plugin(clippy)]

extern crate test;
extern crate primal;
extern crate stopwatch;
extern crate num;

pub mod problem01 {
    pub fn main() -> u32 {
        (1..1000)
            .filter(|num| num % 3 == 0 || num % 5 == 0)
            .fold(0, |sum, value| sum + value)
    }
}

pub mod problem02 {
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

pub mod problem03 {
    use primal;

    pub fn main() -> usize {
        let mut num = 600851475143;

        for prime in primal::Primes::all() {
            if prime == 1 {
                continue;
            }

            if num % prime == 0 {
                num /= prime;

                if num == 1 {
                    return prime;
                }
            }
        }

        0
    }
}

pub mod problem04 {
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

pub mod problem05 {
    use primal;
    use std::collections::HashMap;

    pub fn main() -> usize {
        let sieve = primal::Sieve::new(7);

        let mut hashmap = HashMap::new();
        for i in 2..21 {
            for (prime, mut exponent) in sieve.factor(i).unwrap() {
                let entry = hashmap.entry(prime).or_insert(exponent);
                if entry < &mut exponent {
                    *entry = exponent;
                }
            }
        }

        hashmap.iter()
            .map(|(prime, exponent)| {
                prime.pow(*exponent as u32)
            })
            .fold(1, |sum, c| sum * c)
    }
}

pub mod problem06 {
    use num;
    use num::bigint::ToBigUint;

    pub fn main() -> num::BigUint {
        let mut sum_of_squares: num::BigUint = num::Zero::zero();
        let mut square_of_sums: num::BigUint = num::Zero::zero();

        for i in 1..101u64 {
            sum_of_squares = sum_of_squares + i.pow(2).to_biguint().unwrap();
            square_of_sums = square_of_sums + i.to_biguint().unwrap();
        };

        square_of_sums = num::pow(square_of_sums, 2);

        square_of_sums - sum_of_squares
    }
}

pub mod problem07 {
    use primal;

    pub fn main() -> usize {
        primal::StreamingSieve::nth_prime(10_001)
    }
}

pub mod problem08 {
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
            }
        };

        largest
    }
}

pub mod problem09 {
    pub fn main() -> u64 {
        let s = 1000u64;

        for a in 1..s {
            for b in 1..s - a {
                let c = s - a - b;
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    return a * b * c;
                }
            }
        }
        0
    }
}

pub mod problem10 {
    use primal;

    pub fn main() -> usize {
        primal::Primes::all()
            .take_while(|n| *n <= 2_000_000)
            .fold(0, |a, b| (a as usize) + (b as usize))
    }
}

pub mod problem11 {
    use std::cmp;

    #[allow(needless_range_loop)]
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

        largest_product
    }
}

pub mod problem12 {
    use primal::*;

    pub struct TriangleNums {
        curr: u64,
        index: u64
    }

    impl Iterator for TriangleNums {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            self.index = self.index + 1;
            self.curr = self.curr + self.index;

            Some(self.curr)
        }
    }

    pub fn triangle_nums() -> TriangleNums {
        TriangleNums { curr: 0, index: 0 }
    }

    fn num_divisors(n: u64, sieve: &Sieve) -> usize {
        sieve.factor(n as usize)
            .unwrap()
            .into_iter()
            .map(|(_, x)| x)
            .fold(1, |acc, x| acc * (x + 1))
    }

    pub fn main() -> u64 {
        // Hopefully 1 thousand primes is enough.
        // We currently do no checking to ensure this;
        // and num_divisors will panic on unwrapping sieve.factor
        // if this isn't high enough.
        let (_, hi) = estimate_nth_prime(1000);
        let sieve = Sieve::new(hi as usize);

        for num in triangle_nums() {
            let factor_amount = num_divisors(num, &sieve);

            if factor_amount > 500 {
                return num;
            }
        }

        0
    }
}

pub mod problem13 {
    use num;
    const NUMS: [&'static str; 100] = [
        "37107287533902102798797998220837590246510135740250",
        "46376937677490009712648124896970078050417018260538",
        "74324986199524741059474233309513058123726617309629",
        "91942213363574161572522430563301811072406154908250",
        "23067588207539346171171980310421047513778063246676",
        "89261670696623633820136378418383684178734361726757",
        "28112879812849979408065481931592621691275889832738",
        "44274228917432520321923589422876796487670272189318",
        "47451445736001306439091167216856844588711603153276",
        "70386486105843025439939619828917593665686757934951",
        "62176457141856560629502157223196586755079324193331",
        "64906352462741904929101432445813822663347944758178",
        "92575867718337217661963751590579239728245598838407",
        "58203565325359399008402633568948830189458628227828",
        "80181199384826282014278194139940567587151170094390",
        "35398664372827112653829987240784473053190104293586",
        "86515506006295864861532075273371959191420517255829",
        "71693888707715466499115593487603532921714970056938",
        "54370070576826684624621495650076471787294438377604",
        "53282654108756828443191190634694037855217779295145",
        "36123272525000296071075082563815656710885258350721",
        "45876576172410976447339110607218265236877223636045",
        "17423706905851860660448207621209813287860733969412",
        "81142660418086830619328460811191061556940512689692",
        "51934325451728388641918047049293215058642563049483",
        "62467221648435076201727918039944693004732956340691",
        "15732444386908125794514089057706229429197107928209",
        "55037687525678773091862540744969844508330393682126",
        "18336384825330154686196124348767681297534375946515",
        "80386287592878490201521685554828717201219257766954",
        "78182833757993103614740356856449095527097864797581",
        "16726320100436897842553539920931837441497806860984",
        "48403098129077791799088218795327364475675590848030",
        "87086987551392711854517078544161852424320693150332",
        "59959406895756536782107074926966537676326235447210",
        "69793950679652694742597709739166693763042633987085",
        "41052684708299085211399427365734116182760315001271",
        "65378607361501080857009149939512557028198746004375",
        "35829035317434717326932123578154982629742552737307",
        "94953759765105305946966067683156574377167401875275",
        "88902802571733229619176668713819931811048770190271",
        "25267680276078003013678680992525463401061632866526",
        "36270218540497705585629946580636237993140746255962",
        "24074486908231174977792365466257246923322810917141",
        "91430288197103288597806669760892938638285025333403",
        "34413065578016127815921815005561868836468420090470",
        "23053081172816430487623791969842487255036638784583",
        "11487696932154902810424020138335124462181441773470",
        "63783299490636259666498587618221225225512486764533",
        "67720186971698544312419572409913959008952310058822",
        "95548255300263520781532296796249481641953868218774",
        "76085327132285723110424803456124867697064507995236",
        "37774242535411291684276865538926205024910326572967",
        "23701913275725675285653248258265463092207058596522",
        "29798860272258331913126375147341994889534765745501",
        "18495701454879288984856827726077713721403798879715",
        "38298203783031473527721580348144513491373226651381",
        "34829543829199918180278916522431027392251122869539",
        "40957953066405232632538044100059654939159879593635",
        "29746152185502371307642255121183693803580388584903",
        "41698116222072977186158236678424689157993532961922",
        "62467957194401269043877107275048102390895523597457",
        "23189706772547915061505504953922979530901129967519",
        "86188088225875314529584099251203829009407770775672",
        "11306739708304724483816533873502340845647058077308",
        "82959174767140363198008187129011875491310547126581",
        "97623331044818386269515456334926366572897563400500",
        "42846280183517070527831839425882145521227251250327",
        "55121603546981200581762165212827652751691296897789",
        "32238195734329339946437501907836945765883352399886",
        "75506164965184775180738168837861091527357929701337",
        "62177842752192623401942399639168044983993173312731",
        "32924185707147349566916674687634660915035914677504",
        "99518671430235219628894890102423325116913619626622",
        "73267460800591547471830798392868535206946944540724",
        "76841822524674417161514036427982273348055556214818",
        "97142617910342598647204516893989422179826088076852",
        "87783646182799346313767754307809363333018982642090",
        "10848802521674670883215120185883543223812876952786",
        "71329612474782464538636993009049310363619763878039",
        "62184073572399794223406235393808339651327408011116",
        "66627891981488087797941876876144230030984490851411",
        "60661826293682836764744779239180335110989069790714",
        "85786944089552990653640447425576083659976645795096",
        "66024396409905389607120198219976047599490197230297",
        "64913982680032973156037120041377903785566085089252",
        "16730939319872750275468906903707539413042652315011",
        "94809377245048795150954100921645863754710598436791",
        "78639167021187492431995700641917969777599028300699",
        "15368713711936614952811305876380278410754449733078",
        "40789923115535562561142322423255033685442488917353",
        "44889911501440648020369068063960672322193204149535",
        "41503128880339536053299340368006977710650566631954",
        "81234880673210146739058568557934581403627822703280",
        "82616570773948327592232845941706525094512325230608",
        "22918802058777319719839450180888072429661980811197",
        "77158542502016545090413245809786882778948721859617",
        "72107838435069186155435662884062257473692284509516",
        "20849603980134001723930671666823555245252804609722",
        "53503534226472524250874054075591789781264330331690",
    ];

    pub fn main() -> u64 {
        assert_eq!(NUMS.len(), 100);

        let mut sum: num::BigUint = num::Zero::zero();
        for num in NUMS.into_iter() {
            assert_eq!(num.len(), 50);
            sum = sum + num.parse::<num::BigUint>().unwrap();
        }

        let mut str_sum = sum.to_string();
        str_sum.truncate(10);
        str_sum.parse::<u64>().unwrap()
    }
}

pub mod problem14 {
    pub fn main() -> u64 {
        let mut max_length = 0;
        let mut max_num = 1;

        for start_num in (1..1_000_000).filter(|x| x % 2 > 0) {
            let mut current_num: u64 = start_num;
            let mut iterations = 1;

            while current_num != 1 {
                current_num = if current_num % 2 == 0 {
                    current_num / 2
                } else {
                    current_num * 3 + 1
                };

                iterations += 1;
            }

            if iterations > max_length {
                max_length = iterations;
                max_num = start_num;
            }
        }

        max_num
    }
}

pub mod problem15 {
    pub fn main() -> u64 {
        let size = 20;

        let n = size * 2;
        let k = size;

        // The non-ideal way of doing this. In reality, this is just
        // `(2*size)! / size!`; but that causes integer overflow.
        // 40! isn't small.
        (0..k).zip(1..(k + 1))
            .fold(1.0, |r, (a, b)| r * ((n - a) as f64 / b as f64)) as u64
    }
}

pub mod problem16 {
    use num;
    use num::bigint::ToBigUint;

    pub fn main() -> u32 {
        let r = num::checked_pow(2.to_biguint().unwrap(), 1000).unwrap();

        r.to_str_radix(10).chars().fold(0, |r, c| r + c.to_digit(10).unwrap())
    }
}

/// Unimplemented in code due to problem consisting of
/// code that requires extensive matching. Eventual
/// implementation will happen, but not as of yet.
pub mod problem17 {
    pub fn main() -> u64 { 0 }
}

pub mod problem18 {
    use std::cmp;

    fn next_row_indices(index: usize) -> (usize, usize) {
        (index, index + 1)
    }

    fn next_row_values(index: usize, row: &[u64]) -> (u64, u64) {
        let indices = next_row_indices(index);
        (row[indices.0], row[indices.1])
    }

    pub fn main() -> u64 {
        let mut input: Vec<Vec<u64>> = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 04, 82, 47, 65],
            vec![19, 01, 23, 75, 03, 34],
            vec![88, 02, 77, 73, 07, 63, 67],
            vec![99, 65, 04, 28, 06, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]
        ];

        let mut previous_row = input.pop();
        while let Some(row) = input.pop() {
            let mut new_row = Vec::new();

            if let Some(previous_row) = previous_row {
                for (index, number) in row.iter().enumerate() {
                    let next_row_vals = next_row_values(index, &previous_row);

                    let sum = number + cmp::max(next_row_vals.0, next_row_vals.1);

                    new_row.push(sum);
                };

            }

            previous_row = Some(new_row);
        };

        previous_row.unwrap()[0]
    }
}

pub mod problem19 {
    #[derive(Debug, PartialEq, Eq)]
    enum Day {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }

    impl Day {
        fn next(&self) -> Self {
            match *self {
                Day::Sunday => Day::Monday,
                Day::Monday => Day::Tuesday,
                Day::Tuesday => Day::Wednesday,
                Day::Wednesday => Day::Thursday,
                Day::Thursday => Day::Friday,
                Day::Friday => Day::Saturday,
                Day::Saturday => Day::Sunday
            }
        }
    }

    pub fn main() -> u64 {
        let mut day_of_week = Day::Monday;

        let mut first_saturdays = 0;

        for year in 1900...2000 {
            let is_leap = year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0);

            // Jan1 Feb2 Mar3 Apr4 May5 Jun6 Jul7 Aug8 Sep9 Oct10 Nov11 Dec12
            for month in 1...12 {
                let number_of_days = if month == 2 {
                    if is_leap {
                        29
                    } else {
                        28
                    }
                } else if month == 9 || month == 4 || month == 6 || month == 11 {
                    30
                } else {
                    31
                };

                let mut day = 1;

                if year != 1900 && day_of_week == Day::Saturday {
                    first_saturdays += 1;
                }

                day += 1;

                while number_of_days - day > 7 {
                    day += 7;
                }

                while day <= number_of_days {
                    day_of_week = day_of_week.next();
                    day += 1;
                }
            }
        }

        first_saturdays
    }
}

pub mod problem20 {
    use num::bigint::ToBigUint;

    pub fn main() -> u32 {
        let mut r = 1.to_biguint().unwrap();
        for i in 1...100 {
            r = r * i.to_biguint().unwrap();
        }

        r.to_str_radix(10).chars().fold(0, |r, c| r + c.to_digit(10).unwrap())
    }
}

pub mod problem_x {
    pub fn main() -> u64 {
        1
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    macro_rules! benchmark {
        ($toRun: ident) => (
            #[bench]
            #[allow(redundant_closure)]
            fn $toRun(b: &mut Bencher) {
                b.iter(|| $toRun::main());
            }
        )
    }

    #[test]
    fn problem12_trianglenums() {
        let mut iter = problem12::triangle_nums();

        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 6);
        assert_eq!(iter.next().unwrap(), 10);
        assert_eq!(iter.next().unwrap(), 15);
    }

    benchmark!(problem01);
    benchmark!(problem02);
    benchmark!(problem03);
    benchmark!(problem04);
    benchmark!(problem05);
    benchmark!(problem06);
    benchmark!(problem07);
    benchmark!(problem08);
    benchmark!(problem09);
    benchmark!(problem10);
    benchmark!(problem11);
    benchmark!(problem12);
    benchmark!(problem13);
    benchmark!(problem14);
    benchmark!(problem15);
    benchmark!(problem16);
    benchmark!(problem17);
    benchmark!(problem18);
    benchmark!(problem19);
    benchmark!(problem20);
}

use stopwatch::{Stopwatch};
fn main() {
    macro_rules! run_problem {
        ($toRun: ident) => (
            {
                let sw = Stopwatch::start_new();
                let result = $toRun::main();
                println!("{}: {} in {}ms", stringify!($toRun), result, sw.elapsed_ms());
            }
        )
    }

    run_problem!(problem01);
    run_problem!(problem02);
    run_problem!(problem03);
    run_problem!(problem04);
    run_problem!(problem05);
    run_problem!(problem06);
    run_problem!(problem07);
    run_problem!(problem08);
    run_problem!(problem09);
    run_problem!(problem10);
    run_problem!(problem11);
    run_problem!(problem12);
    run_problem!(problem13);
    run_problem!(problem14);
    run_problem!(problem15);
    run_problem!(problem16);
    run_problem!(problem17);
    run_problem!(problem18);
    run_problem!(problem19);
    run_problem!(problem20);
}
