fn main() {
    println!("Hello, world!");

    let code_css = "
	body {
		display: flex;
		flex-direction: column;
		min-height: 100%;
	}";

    valid_braces(code_css);
}

fn html_gen(code_css: String) {
    // entity
    if code_css.contains("body") {
        println!("<body>");
    }
}

/// Check if braces are valid.
fn valid_braces<'a>(s: &'a str) -> bool {
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        if c == '}' {
            if !stack.is_empty() && stack.last() == Some(&'{') {
                stack.pop();
            } else {
                return false;
            }
        } else {
            // Add the open brace to the stack.
            stack.push(c);
        }
    }
    if stack.is_empty() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn one_open_brace() {
        assert_eq!(super::valid_braces("{"), false);
    }

    #[test]
    fn one_closed_brace() {
        assert_eq!(super::valid_braces("}"), false);
    }

    #[test]
    fn two_open_braces() {
        assert_eq!(super::valid_braces("{{"), false);
    }

    #[test]
    fn two_closed_braces() {
        assert_eq!(super::valid_braces("}}"), false);
    }

    #[test]
    fn one_brace_pair() {
        assert_eq!(super::valid_braces("{}"), true);
    }

    #[test]
    fn two_brace_pairs() {
        assert_eq!(super::valid_braces("{}{}"), true);
    }
}
