// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 靠拢型双指针
pub fn is_palindrome1(s: String) -> bool {
    // 根据题目要求, 过滤一下字符串
    let s: String = s
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if s.is_empty() {
        return true;
    }

    // 使用双指针来判断
    let mut left = 0;
    let mut right = s.len() - 1;
    let bytes = s.as_bytes();
    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

// 靠拢型双指针, 但不对字符串预处理
pub fn is_palindrome2(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = chars.len() - 1;
    while left < right {
        // 忽略非字符数字
        while left < right && !chars[left].is_ascii_alphanumeric() {
            left += 1;
        }
        // 忽略非字符数字
        while left < right && !chars[right].is_ascii_alphanumeric() {
            right -= 1;
        }

        if !chars[left].eq_ignore_ascii_case(&chars[right]) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

// 使用回文字串的性质: 反转之后依然相同
pub fn is_palindrome3(s: String) -> bool {
    let s: String = s
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    s.chars().rev().collect::<String>() == s
}

// 替换字符串过滤方法, 其实还要慢一些
pub fn is_palindrome4(s: String) -> bool {
    let s: String = s
        .replace(|c: char| !c.is_ascii_alphanumeric(), "")
        .to_ascii_lowercase();
    s.chars().rev().collect::<String>() == s
}

pub type SolutionFn = fn(String) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&str, bool)] = &[
        ("A man, a plan, a canal: Panama", true),
        ("race a car", false),
        (" ", true),
        ("0P", false),
    ];
    for record in RECORDS {
        assert_eq!(func(record.0.to_owned()), record.1);
    }
}

fn main() {
    check_solution(is_palindrome1);
    check_solution(is_palindrome2);
    check_solution(is_palindrome3);
    check_solution(is_palindrome4);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_palindrome1, is_palindrome2, is_palindrome3, is_palindrome4};

    #[test]
    fn test_is_palindrome1() {
        check_solution(is_palindrome1);
    }

    #[test]
    fn test_is_palindrome2() {
        check_solution(is_palindrome2);
    }

    #[test]
    fn test_is_palindrome3() {
        check_solution(is_palindrome3);
    }

    #[test]
    fn test_is_palindrome4() {
        check_solution(is_palindrome4);
    }
}
