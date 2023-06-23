#![allow(dead_code)]
/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/

fn valid_paren_str(s: &str) -> bool {
    for c in s.chars() {
        match c {
            '{' => continue,
            '[' => continue,
            '(' => continue,
            ')' => continue,
            '}' => continue,
            ']' => continue,
            _ => return false,
        }
    }
    true
}

fn validate_paren(s: &str) -> bool {
    if !valid_paren_str(s) {
        return false;
    }

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            _ => {
                if stack.is_empty() {
                    return false;
                }
                if stack.last() != Some(&c) {
                    return false;
                }
                stack.pop();
            }
        }
    }
    return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
    }

    fn it_works2() {
        assert_eq!(valid_paren_str("()"), true);
        assert_eq!(valid_paren_str("()[]{}"), true);
        assert_eq!(valid_paren_str("({[]()})"), true);
        assert_eq!(valid_paren_str("(}"), true);
        assert_eq!(valid_paren_str("()]"), true);
        assert_eq!(valid_paren_str("(){"), true);
        assert_eq!(valid_paren_str("(q){"), false);
        assert_eq!(valid_paren_str(""), false);
        assert_eq!(valid_paren_str("222"), false);
    }
}
