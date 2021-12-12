fn main() {
    println!("Hello, world!");

    let code_css = "
	body {
		display: flex;
		flex-direction: column;
		min-height: 100%;
	}"
    .to_string();

    assert!(valid_braces(&code_css));
    println!("{}", html_gen(code_css));
}

fn html_gen(mut s: String) -> String {
    // entity
    let entities = vec!["body", "html"];
    let tag_open_brace = " {";
    for entity in entities {
        let entity = entity.to_owned() + tag_open_brace;
        s = s.replace(&entity, &html_tag_with_open_brace("body"));
    }
    s
}

fn html_tag_with_open_brace(s: &str) -> String {
    format!("<span class=\"tag\">{}</span> {{", s)
}

// HTML generator for a single tag
fn html_tag(s: &str) -> String {
    format!("<span class=\"tag\">{}</span>", s)
}

/// Check if braces are valid.
fn valid_braces<'a>(s: &'a str) -> bool {
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '}' => {
                if !stack.is_empty() && stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            // Add the open brace to the stack.
            '{' => stack.push(c),
            _ => (),
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

    #[test]
    fn one_brace_pair_with_word() {
        assert_eq!(super::valid_braces("{hi}"), true);
    }

    #[test]
    fn html_html_tag() {
        assert_eq!(super::html_tag("html"), "<span class=\"tag\">html</span>");
    }

    #[test]
    fn html_body_tag() {
        assert_eq!(super::html_tag("body"), "<span class=\"tag\">body</span>");
    }

    #[test]
    fn html_main_tag() {
        assert_eq!(super::html_tag("main"), "<span class=\"tag\">main</span>");
    }

    #[test]
    fn html_body_tag_with_open_brace() {
        assert_eq!(
            super::html_tag_with_open_brace("body"),
            "<span class=\"tag\">body</span> {"
        );
    }
}
