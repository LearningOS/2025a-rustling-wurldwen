// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end = input.len();
    
    // 正向遍历：找到第一个非空格字符的索引
    for (i, ch) in input.chars().enumerate() {
        if ch != ' ' {
            start = i;
            break;
        }
    }
    
    // 反向遍历：找到最后一个非空格字符的索引
    for (i, ch) in input.chars().rev().enumerate() {
        if ch != ' ' {
            end = input.len() - i;
            break;
        }
    }
    
    // 如果 start >= end，说明全是空格，返回空字符串
    if start >= end {
        String::new()
    } else {
        input[start..end].to_string()
    }
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
