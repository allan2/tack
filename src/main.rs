use tack::coloured_html;

fn main() {
    let a = "
body {
	display: flex;
	flex-direction: column;
	min-height: 100%;
}"
    .to_string();

    let b = coloured_html(a);
    println!("{}", b);
}
