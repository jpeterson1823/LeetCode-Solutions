struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // Zero Case
        if n == 0 { return 1.0; }

        // Declare vars
        let ans: f64;
        let mut p = n;
        let mut v = x;

        // convert to frac if pow is neg
        if p < 0 {
            p = -p;
            v = 1.0/v;
        }
        
        // use quick-maths to speed up calculation
        if n % 2 == 0 {
            ans = (v*v).powi(p/2);
        } else {
            ans = v*(v*v).powi(p/2);
        }

        // make sure answer is finite before returning
        if ans.is_finite() {
            ans
        } else {
            0.0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
