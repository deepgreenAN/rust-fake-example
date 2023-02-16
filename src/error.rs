#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),

    #[error("Invalid youtube url.")]
    InvalidYoutubeUrlError,
}
