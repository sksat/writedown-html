pub use writedown;

pub mod html;
pub use html::Node;

pub fn from_str(s: &str) -> Result<Node, ()> {
    let ast = writedown::parse(s);
    if let Ok(ast) = ast {
        Ok(ast.into())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use writedown::Render;

    #[test]
    fn simple() {
        let src = r#"sentence0
= title level 1

hogehoge fugafuga
aaaaaaaaaaaaaaaaa

== title level 2
hvoeahovea
"#;
        let ast = writedown::parse(src).unwrap();
        let html: html::Node = ast.into();
        let s = html.render();

        println!("{}", s);
    }
}
