struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest = String::new();
        let mut index = 0;
        let test = strs.clone();

        loop {
            let cc = match test[0].chars().nth(index) {
                Some(c) => c,
                None => break,
            };

            print!("[");
            for s in test.clone() {
                print!("{} ", s);
                match s.chars().nth(index) {
                    Some(c) => {
                        if c != cc {
                            return longest;
                        }
                    }
                    None => { return longest; },
                };
            }
            println!("\x08\x08]");

            longest.push(cc);
            index += 1;
            println!("LOOP");
        }

        longest
    }
}

fn main() {
    println!("Tested on LeetCode's internal editor.");
}
