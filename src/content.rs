pub mod keyword;
pub mod header;
pub mod body;
mod regexes;

#[derive(Debug)]
pub struct ArxivPaperContent {
    /// arxiv id to reference later
    pub id: String,
    /// abstract content
    pub abstract_text: String,
    /// keywords
    pub keywords: Vec<String>,
    /// the actual content of the paper, intro through the end
    pub paper_content: String,
}
