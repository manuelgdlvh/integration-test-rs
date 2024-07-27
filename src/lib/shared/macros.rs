#[macro_export]
macro_rules! deserialize_or_bail {
    ($payload:expr) => {
        match $payload.deserialize() {
            Ok(data) => data,
            Err(e) => return e.into_response(),
        }
    };
}