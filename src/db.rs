use crate::err::AppResult;
use crate::metadata::ArxivMetadata;
use rusqlite::{named_params, Connection, Transaction};
use std::path::Path;

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
    
    pub fn insert_metadata(&self, metadata: ArxivMetadata) -> AppResult<()> {
        let item = metadata;
        let mut stmt = self.conn.prepare_cached("INSERT INTO arxiv_metadata VALUES (:id,:submitted,:authors,:title,:comments,:journal_ref,:doi,:categories)")?;
        let params = named_params! {
            ":id": item.id(),
            ":submitted": item.submitter(),
            ":authors": item.authors(),
            ":title": item.title(),
            ":comments": item.comments(),
            ":journal_ref": item.journal_ref(),
            ":doi": item.doi(),
            ":categories": item.categories()
        };

        stmt.execute(params)?;
        
        Ok(())
    }
}
