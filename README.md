# ArchiveForge

A fast, content-addressed document catalog derived from the useful ideas in
`E:\TheSWShop\DocMgr`. The initial core computes SHA-256 identities, prevents
duplicate storage, and catalogs neutral metadata. No legacy content or BLOB data
is copied.

```powershell
cargo test
cargo run -- path\to\document.pdf
```

Next slices: durable SQLite metadata, categories/tags, OCR adapters, previews,
retention rules, version chains, and pluggable local/object storage.
