use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_TRIPLE: Regex =
        Regex::new(r"(abc|bcd|cde|def|efg|fgh|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz)").unwrap();
    static ref REGEX_PAIR: Regex = Regex::new(r"([a-z])\1.*([a-z])\2").unwrap();
    static ref REGEX_BAD: Regex = Regex::new(r"(i|l|o)").unwrap();
}

/// Password struct used to generate valid passwords based on the correctness rules. Described in
/// the AOC 2015 Day 11 problem (<https://adventofcode.com/2015/day/11>).
pub struct PasswordGenerator {
    chars: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(chars: &[char]) -> PasswordGenerator {
        PasswordGenerator {
            chars: chars.to_vec(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.is_empty() {
            self.chars = vec!['a'];
        }
        loop {
            let mut i = self.chars.len() - 1;
            // Increment chars to find the next candidate password
            loop {
                // Increment password
                if self.chars[i] != 'z' {
                    self.chars[i] = char::from(self.chars[i] as u8 + 1);
                    break;
                } else {
                    self.chars[i] = 'a';
                    if i == 0 {
                        self.chars.insert(0, 'a');
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
            // Check if password is valid
            let candidate = self.chars.iter().collect::<String>();
            if let Ok(Some(caps)) = REGEX_PAIR.captures(&candidate) {
                if caps[1] == caps[2] {
                    continue;
                }
            } else {
                continue;
            }
            if REGEX_TRIPLE.is_match(&candidate).unwrap()
                && !REGEX_BAD.is_match(&candidate).unwrap()
            {
                return Some(candidate);
            }
        }
    }
}