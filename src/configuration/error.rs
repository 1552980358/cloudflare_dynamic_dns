pub enum Error {
    CloudflareImportFail,
    ConfigImportFail
    CloudflareImportFail(String),
}