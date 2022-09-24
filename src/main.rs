use latex2mathml::{latex_to_mathml, DisplayStyle};

fn main() {
    let latex = r#"x^2"#;
    let latex2 = r#"x^2 + 5x -3 = 2x + 5"#;
    let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();
    let mathml2 = latex_to_mathml(latex2, DisplayStyle::Block).unwrap();
    println!("{}, {}", mathml, mathml2);
}
