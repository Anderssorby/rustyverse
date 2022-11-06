use juniper::FieldError;

/// Logging and error reporing module

/// Handle and print a generic error
pub fn handle_err(e: impl std::fmt::Debug) -> anyhow::Error {
  error!("{:#?}", e);
  anyhow::format_err!("{:#?}", e)
}

/// Handle and print an opaque field error.
/// Usefull for dealing with common errors in GraphQL without leaking info.
pub fn opaque_field_error(e: impl std::fmt::Debug) -> FieldError {
  error!("{:#?}", e);
  FieldError::from(format!("{:#?}", e))
}
