struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut a = Vec::new();
        if n == 0 {
            a.push("".to_string());
        } else {
            for i in 0..n {
                for left in Solution::generate_parenthesis(i) {
                    for right in Solution::generate_parenthesis(n-1-i) {
                        let mut s = left.clone();
                        s.push('(');
                        s.push_str(right.as_str());
                        s.push(')');
                        a.push(s);
                    }
                }
            }
        }

        a
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
