use writedown::{ast, Render};

//TODO: implement by macro

// ast::Node -> html::Node
// html::Node(v).render();

#[derive(Debug)]
pub enum Node {
    Section(Section),
    Paragraph(Paragraph),
    Unknown,
}

impl Render for Node {
    fn render(&self) -> String {
        match &self {
            Node::Section(s) => s.render(),
            Node::Paragraph(p) => p.render(),
            _ => unimplemented!(),
        }
    }
}

impl From<ast::Node> for Node {
    fn from(node: ast::Node) -> Self {
        match node {
            ast::Node::Section(v) => Node::Section(v.into()),
            ast::Node::Paragraph(v) => Node::Paragraph(v.into()),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Section {
    pub level: usize,
    pub title: String,
    pub child: Vec<Node>,
}
#[derive(Debug)]
pub struct Paragraph {
    pub child: Vec<ParagraphChild>,
}
#[derive(Debug)]
pub enum ParagraphChild {
    Sentence(String),
}

impl From<ast::Section> for Section {
    fn from(v: ast::Section) -> Section {
        let mut child: Vec<Node> = Vec::new();
        for c in v.child {
            child.push(c.into());
        }
        Section {
            level: v.level,
            title: v.title,
            child,
        }
    }
}
impl From<ast::Paragraph> for Paragraph {
    fn from(v: ast::Paragraph) -> Paragraph {
        let mut child = Vec::new();
        for c in v.child {
            child.push(c.into());
        }
        Paragraph { child }
    }
}
impl From<ast::ParagraphChild> for ParagraphChild {
    fn from(v: ast::ParagraphChild) -> ParagraphChild {
        match v {
            ast::ParagraphChild::Sentence(s) => ParagraphChild::Sentence(s),
        }
    }
}

impl Render for Section {
    fn render(&self) -> String {
        let l = format!("{}", self.level + 1);
        let mut cs = String::new();
        for c in &self.child {
            cs += &c.render();
        }

        format!("<h{}>{}</h{}>\n{}", l, self.title, l, cs)
    }
}
impl Render for Paragraph {
    fn render(&self) -> String {
        let mut cs = String::new();
        for c in &self.child {
            cs += &c.render();
        }
        format!("<p>{}</p>\n", cs)
    }
}
impl Render for ParagraphChild {
    fn render(&self) -> String {
        match &self {
            ParagraphChild::Sentence(s) => s.to_string(),
        }
    }
}
