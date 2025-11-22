use liberror::AnyError;
use poem_openapi::Object;

#[derive(Object)]
pub struct ApiError {
    // TODO: more / can I squeeze arbitrary json into an Object?
    message: String,
}
impl<E: Into<AnyError>> From<E> for ApiError {
    fn from(value: E) -> Self {
        let norm = value.into();

        Self {
            message: norm.to_string(),
        }
    }
}
