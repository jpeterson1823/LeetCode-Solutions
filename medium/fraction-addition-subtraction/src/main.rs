struct Solution;

use std::cmp;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        fn get_components(exp: String) -> Vec<Fraction> {
            // split additions
            let additions: Vec<String> = exp.split("+")
                                            .map(|s| s.to_string())
                                            .collect();

            // look at each addition and split into components
            let mut components = Vec::new();

            for addition in additions {
                match addition.find('-') {
                    Some(_) => {
                        if addition.chars().nth(0).unwrap() == '-' {
                            let mut temp = addition[1..].to_string();
                            temp = temp.replace('-', "+-");
                            temp.insert(0,'-');
                            let tcomp = get_components(temp);
                            for tc in tcomp { components.push(tc); }
                        } else {
                            let mut temp: Vec<String> = addition.split('-')
                                                                .map(|s| s.to_string())
                                                                .collect();
                            temp[1].insert(0, '-');
                            for t in temp {
                                components.push(Fraction::new(t));
                            }
                        }
                    },
                    None => components.push(Fraction::new(addition)),
                };
            };

            components
        }

        let components = get_components(expression);
        let mut answer = Fraction::new("0/1".to_string());
        for c in components {
            answer = answer + c;
        }
        answer.to_string()
    }
}

#[derive(Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(fstr: String) -> Self {
        println!("Fraction from \"{}\"",fstr);
        let nums: Vec<i32> = fstr.split("/")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
        let mut n = Self {
            numerator:   nums[0],
            denominator: nums[1],
        };
        n.reduce();
        n
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.numerator.to_string().as_str());
        s.push('/');
        s.push_str(self.denominator.to_string().as_str());
        s
    }

    pub fn reduce(&mut self) {
        if self.numerator != 0 {
            let gcd = Self::find_gcd(self.numerator, self.denominator);
            self.numerator /= gcd;
            self.denominator /= gcd;
        }
    }

    fn find_gcd(a: i32, b: i32) -> i32 {
        if a != 0 && b != 0 {
            let mut max = cmp::max(a.abs(), b.abs());
            let mut min = cmp::min(a.abs(), b.abs());
            loop {
                let res = max % min;
                if res == 0 { return min; }
                max = min;
                min = res;
            }
        } else {
            1
        }
    }

    fn find_lcd(a: i32, b: i32) -> i32 {
        a * b / Self::find_gcd(a, b)
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcd = Self::find_lcd(self.denominator, other.denominator);
        print!("lcd: {} \t| ", lcd);
        let self_numerator = self.numerator * (lcd / self.denominator);
        print!("snum: {} \t| ", self_numerator);
        let other_numerator = other.numerator * (lcd / other.denominator);
        print!("onum: {} \t| ", other_numerator);
        let mut n = Self {
            numerator: self_numerator + other_numerator,
            denominator: lcd,
        };
        if n.numerator == 0 {
            n.denominator = 1;
        } else {
            n.reduce();
        }
        println!("{} + {} = {}", self.to_string(), other.to_string(), n.to_string());
        n
    }
}

fn main() {
    println!("{}", Solution::fraction_addition("-1/2+1/2+1/2+1/4".to_string()));
    println!("{}", Solution::fraction_addition("-4/7-3/4+2/3".to_string()));
}
