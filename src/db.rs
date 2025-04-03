pub mod pages;

use crate::err::{AppResult};
use crate::metadata::{ArxivMetadata, ArxivVersion};
use rusqlite::{named_params, Connection, Statement, Transaction};
use std::path::Path;
use crate::content::ArxivPaperContent;
use crate::db::pages::QueryPage;
use crate::extraction::{ExtractError, ExtractResultRecord};

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
    
    pub fn turn_off_synchronous(&self) -> AppResult<()> {
        self.conn.pragma_update(None, "synchronous", "OFF")?;
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

    pub fn count_arxiv_ids(&self) -> AppResult<u64> {
        let result = self.conn.query_row(
            "SELECT COUNT(id) FROM arxiv_metadata",
            [],
            |row| row.get::<_, u64>(0)
        )?;

        Ok(result)
    }

    pub fn select_arxiv_ids(&self, page: QueryPage) -> AppResult<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT id FROM arxiv_metadata LIMIT :limit OFFSET :offset")?;
        let params = named_params! {
            ":limit": page.limit,
            ":offset": page.offset,
        };
        
        let ids = stmt
            .query_map(params, |row| row.get::<_, String>("id"))?
            .filter_map(|id| id.ok())
            .collect::<Vec<_>>();

        Ok(ids)
    }
    
    pub fn sample_arxiv_ids(&self, count: u64) -> AppResult<Vec<String>> {
        let stmt = self.conn.prepare(r"
        SELECT id FROM arxiv_metadata ORDER BY random() LIMIT :limit
        ")?;
        
        Self::map_id_query(stmt, count)
    }
    
    pub fn sample_arxiv_ids_unprocessed(&self, count: u64) -> AppResult<Vec<String>> {
        let stmt = self.conn.prepare(r"
        WITH candidates AS (
            SELECT id FROM arxiv_metadata WHERE NOT EXISTS (
                SELECT 1 FROM extraction_result WHERE extraction_result.arxiv_id = arxiv_metadata.id 
            )
        )
        SELECT id ORDER BY RANDOM() LIMIT :count
        ")?;

        Self::map_id_query(stmt, count)
    }
    
    fn map_id_query(mut stmt: Statement, count: u64) -> AppResult<Vec<String>> {
        let params = named_params! { ":limit": count };

        let ids = stmt.query_map(params, |row| row.get::<_, String>("id"))?
            .filter_map(|id| id.ok())
            .collect::<Vec<_>>();

        Ok(ids)
    }
    
    pub fn insert_extraction_result(&self, id: &str, err: Option<ExtractError>) -> AppResult<()> {
        let record = err
            .map(ExtractResultRecord::from)
            .unwrap_or(ExtractResultRecord::success(id));
        
        let mut stmt = self.conn.prepare_cached(r"
        INSERT INTO extraction_result (arxiv_id, status_code, status_msg)
        VALUES (:arxiv_id, :status_code, :status_msg)
        ")?;
        
        let params = named_params! {
            ":arxiv_id": record.arxiv_id,
            ":status_code": record.extract_status,
            ":status_msg": record.extract_msg
        };
        
        stmt.execute(params)?;

        Ok(())
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
    
    pub fn update_keywords_and_content(&self, content: ArxivPaperContent) -> AppResult<()> {
        let mut stmt = self.conn.prepare_cached(r"
        UPDATE paper_data
        SET keywords = :keywords, content = :content
        WHERE arxiv_id = :arxiv_id
        ")?;
        
        let params = named_params! {
            ":keywords": content.keywords.join(","),
            ":content": content.paper_content,
            ":arxiv_id": content.id
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
