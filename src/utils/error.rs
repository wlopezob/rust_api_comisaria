//! Main Crate Error
use mongodb::bson;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),
	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
}