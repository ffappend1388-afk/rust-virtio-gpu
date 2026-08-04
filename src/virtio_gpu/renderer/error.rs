#[derive(Debug)]
pub enum RendererError {
    UploadFailed,
    TransferFailed,
    FlushFailed,
    InvalidResource,
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "renderer error: {:?}", self)
    }
}

impl std::error::Error for RendererError {}
