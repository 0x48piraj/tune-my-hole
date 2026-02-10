use std::{
    path::PathBuf,
};

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};

use crate::domain::normalize_domain;

pub struct DomainStats {
    pub domain: Box<str>,
    pub count: u32,
}

pub fn load_domain_stats(db: &PathBuf) -> Result<Vec<DomainStats>> {
    let conn = Connection::open_with_flags(
        db,
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;

    let mut stmt = conn.prepare(
        r#"
        SELECT domain, COUNT(*) as cnt
        FROM queries
        WHERE domain IS NOT NULL
          AND domain != ''
          AND status = 0
        GROUP BY domain
        "#,
    )?;

    let rows = stmt.query_map([], |row| {
        let domain: String = row.get(0)?;
        let count: u32 = row.get(1)?;

        Ok((domain, count))
    })?;

    let mut out = Vec::new();

    for row in rows {
        let (domain, count) = row?;
        if let Some(d) = normalize_domain(&domain) {
            out.push(DomainStats { domain: d, count });
        }
    }

    Ok(out)
}
