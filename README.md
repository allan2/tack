# tack

[![crates.io](https://img.shields.io/crates/v/tack.svg)](https://crates.io/crates/tack)
[![Released API docs](https://docs.rs/tack/badge.svg)](https://docs.rs/tack)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Tack is a minimal syntax highlighter that outputs HTML.

**Note: this library is alpha and subject to breaking API changes.** 
**More languages are on the way!**

Features:
 - class-based
 - minimal HTML

Supported languages:
 - CSS

 For a more fully-featured syntax highlighter, use [`syntect`](https://github.com/trishume/syntect).


## Examples

```css
body {
  display: flex;
  flex-direction: column;
  min-height: 100%;
}
```

Tack usage:
```rust
fn main() {
    let s = "
body {
display: flex;
min-height: 100%;
}".to_string();

    let html = tack::coloured_html(s);
    println!("{}", html);
}
```

Tack output:
```html
<span class="tag">body</span> {
  <span class="property">display</span>: <span class="string">flex</span>;
  <span class="property">min-height</span>: <span class="numeric">100%</span>;
}
```

`syntect` usage:
```rust
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();

    let s = "
body {
    display: flex;
    min-height: 100%;
}";

    let syntax_ref = ss.find_syntax_by_extension("css").unwrap();
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax_ref, &ss, ClassStyle::Spaced);
    for line in LinesWithEndings::from(s) {
        html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let html = html_generator.finalize();
    println!("{}", html);
}
```

`syntect` output:
```html
<span class="source css">
  <span class="meta selector css"><span class="entity name tag css">body</span> </span><span
    class="meta property-list css"><span class="punctuation section property-list css">{</span>
    <span class="meta property-name css"><span class="support type property-name css">display</span></span><span
      class="punctuation separator key-value css">:</span><span class="meta property-value css"> </span><span
      class="meta property-value css"><span class="support constant property-value css">flex</span></span><span
      class="punctuation terminator rule css">;</span>
    <span class="meta property-name css"><span class="support type property-name css">min-height</span></span><span
      class="punctuation separator key-value css">:</span><span class="meta property-value css"> </span><span
      class="meta property-value css"><span class="constant numeric css">100<span
          class="keyword other unit css">%</span></span></span><span class="punctuation terminator rule css">;</span>
  </span><span class="punctuation section property-list css">}</span></span>
```

