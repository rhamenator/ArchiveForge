use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub digest: String,
    pub name: String,
    pub media_type: String,
    pub category: String,
    pub size: usize,
}

#[derive(Debug, Default)]
pub struct Catalog {
    documents: BTreeMap<String, Document>,
}

impl Catalog {
    pub fn ingest(
        &mut self,
        name: &str,
        media_type: &str,
        category: &str,
        bytes: &[u8],
    ) -> &Document {
        let digest = sha256(bytes);
        self.documents
            .entry(digest.clone())
            .or_insert_with(|| Document {
                digest,
                name: name.trim().to_owned(),
                media_type: media_type.trim().to_ascii_lowercase(),
                category: category.trim().to_owned(),
                size: bytes.len(),
            })
    }

    pub fn find_by_category(&self, category: &str) -> Vec<&Document> {
        self.documents
            .values()
            .filter(|document| document.category.eq_ignore_ascii_case(category))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_identity_deduplicates_renamed_files() {
        let mut catalog = Catalog::default();
        catalog.ingest("scan-a.pdf", "application/pdf", "Invoices", b"same bytes");
        catalog.ingest("renamed.pdf", "application/pdf", "Invoices", b"same bytes");
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog.find_by_category("invoices").len(), 1);
    }
}
