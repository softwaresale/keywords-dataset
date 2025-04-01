use crate::err::AppResult;
use crate::metadata::{ArxivMetadata, ArxivVersion};
use rusqlite::{named_params, Connection, Transaction};
use std::path::Path;
use crate::content::ArxivPaperContent;

pub struct ArxivDB {
    conn: Connection,
} 

impl ArxivDB {
    pub fn open<PathT: AsRef<Path>>(path: PathT) -> AppResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Self {
            conn
        })
    }
    
    pub fn execute_ddl(&self) -> AppResult<()> {
        let ddl_query = include_str!("../sql/ddl.sql");
        self.conn.execute_batch(ddl_query)?;
        Ok(())
    }
    
    pub fn queries(&self) -> ArxivDBQueries {
        ArxivDBQueries::wrap(&self.conn)
    }
    
    pub fn txn(&mut self) -> AppResult<Transaction> {
        self.conn.transaction()
            .map_err(Into::into)
    }
}

pub struct ArxivDBQueries<'a> {
    conn: &'a Connection,
}

impl<'a> ArxivDBQueries<'a> {
    
    pub fn wrap(conn: &'a Connection) -> Self {
        Self {
            conn
        }
    }

    pub fn insert_arxiv_metadata(&self, metadata: ArxivMetadata) -> AppResult<()> {
        let arxiv_id = metadata.id().expect("metadata has null arxiv id");
        self.insert_metadata(&metadata)?;
        self.insert_versions(arxiv_id, metadata.versions())?;
        
        // make a quick content for the abstract
        self.insert_content(arxiv_id, ArxivPaperContent {
            id: "".to_string(),
            abstract_text: metadata.abstract_text().cloned().unwrap_or_default(),
            paper_content: String::default(),
            keywords: Vec::default()
        })?;
        
        Ok(())
    }

    pub fn insert_content(&self, arxiv_id: &str, content: ArxivPaperContent) -> AppResult<()> {
        let mut stmt = self.conn.prepare_cached(r"
        INSERT INTO paper_data (arxiv_id, abstract, keywords, content)
        VALUES (:arxiv_id, :abstract, :keywords, :content)
        ")?;
        
        let params = named_params! {
            ":arxiv_id": arxiv_id,
            ":abstract": content.abstract_text,
            ":keywords": content.keywords.join(","),
            ":content": content.paper_content
        };
        
        stmt.execute(params)?;
        
        Ok(())
    }
    
    pub fn insert_versions(&self, arxiv_id: &str, metadata: &[ArxivVersion]) -> AppResult<()> {
        let mut stmt = self.conn.prepare_cached(r"
        INSERT INTO arxiv_version(arxiv_id, version, created)
        VALUES (:arxiv_id, :version, :created)
        ")?;
        for item in metadata {
            let params = named_params! {
                ":arxiv_id": arxiv_id,
                ":version": item.version(),
                ":created": item.created()
            };

            stmt.execute(params)?;
        }

        Ok(())
    }

    pub fn insert_metadata(&self, metadata: &ArxivMetadata) -> AppResult<()> {
        let mut stmt = self.conn.prepare_cached("INSERT INTO arxiv_metadata VALUES (:id,:submitted,:authors,:title,:comments,:journal_ref,:doi,:categories)")?;
        let params = named_params! {
            ":id": metadata.id(),
            ":submitted": metadata.submitter(),
            ":authors": metadata.authors(),
            ":title": metadata.title(),
            ":comments": metadata.comments(),
            ":journal_ref": metadata.journal_ref(),
            ":doi": metadata.doi(),
            ":categories": metadata.categories()
        };

        stmt.execute(params)?;
        
        Ok(())
    }
}
