use rusqlite::Row;

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

pub struct ArxivPaperContentEntity {
    /// arxiv id to reference later
    pub id: String,
    /// abstract content
    pub abstract_text: String,
    /// keywords
    pub keywords: String,
    /// the actual content of the paper, intro through the end
    pub paper_content: String,
}

impl<'a, 'db> TryFrom<&'a Row<'db>> for ArxivPaperContentEntity {
    type Error = rusqlite::Error;

    fn try_from(value: &'a Row<'db>) -> Result<Self, Self::Error> {
        let id = value.get::<_, String>("arxiv_id")?;
        let abstract_text = value.get::<_, String>("abstract")?;
        let keywords = value.get::<_, String>("keywords")?;
        let paper_content = value.get::<_, String>("content")?;
        
        Ok(Self {
            id,
            abstract_text,
            keywords,
            paper_content,
        })
    }
}
