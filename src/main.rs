use std::fs;

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
    let html = html_gen(code_css);
    fs::write("test.html", html).unwrap();
}

/// Assumptions:
/// no trailing whitespace
/// valid braces
fn html_gen(s: String) -> String {
    let mut new = String::new();
    for line in s.lines() {
        if line.contains("{") {
            // mark up the entity, e.g. a classname or a CSS selector
            let entity = line.split_once(" {").unwrap().0.trim_start();
            let newline = &line.replace(entity, &html_tag(entity));
            new.push_str(newline);
            new.push('\n');
        } else if line.contains(":") {
            let split = line.split_once(':').unwrap();

            // mark up the CSS property
            let property = split.0.trim_start();
            let mut newline = &line.replace(property, &html_property(property));

            // mark up the value
            let mut value = split.1.chars();
            value.next(); // remove leading whitespace that was between colon and value
            value.next_back(); // don't touch trailing semicolon

            let value_first_char = value.next().unwrap();
            let value = value_first_char.to_string() + value.as_str();
            let html_value = if value_first_char.is_numeric() {
                html_value_numeric(&value)
            } else {
                html_value_string(&value)
            };
            let newline = newline.replace(&value, &html_value);
            new.push_str(&newline);
            println!("{}", value);
            new.push('\n');
        } else if line == "}" {
            new.push('}');
            new.push('\n');
        }
    }
    new
}

fn html_tag_with_open_brace(s: &str) -> String {
    format!("<span class=\"tag\">{}</span> {{", s)
}

// HTML generator for a single tag
fn html_tag(s: &str) -> String {
    format!("<span class=\"tag\">{}</span>", s)
}

fn html_property_with_colon(s: &str) -> String {
    format!("<span class=\"property\">{}</span>:", s)
}

// HTML generator for a property
fn html_property(s: &str) -> String {
    format!("<span class=\"property\">{}</span>", s)
}

// HTML generator for a property
fn html_value_numeric(s: &str) -> String {
    format!("<span class=\"numeric\">{}</span>", s)
}

fn html_value_string(s: &str) -> String {
    format!("<span class=\"string\">{}</span>", s)
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

    #[test]
    fn html_height_property_with_colon() {
        assert_eq!(
            super::html_property_with_colon("height"),
            "<span class=\"property\">height</span>:"
        );
    }

    #[test]
    fn html_value_int() {
        assert_eq!(
            super::html_numeric_value("5"),
            "<span class=\"numeric\">5</span>"
        );
    }

    #[test]
    fn html_value_pct() {
        assert_eq!(
            super::html_numeric_value("10%"),
            "<span class=\"numeric\">10%</span>"
        );
    }

    #[test]
    fn html_value_flex() {
        assert_eq!(
            super::html_numeric_value("flex"),
            "<span class=\"string\">flex</span>"
        );
    }
}
