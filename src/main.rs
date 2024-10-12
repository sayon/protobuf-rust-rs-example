mod errors {
    tonic::include_proto!("errors");
}

use errors::{Error, InternalServerError, NotFoundError, ValidationError};

pub fn main() {
    let not_found_error = NotFoundError {
        error: Some(Error {
            message: "Resource not found".to_string(),
            code: 404,
        }),
        resource: "User".to_string(),
    };

    println!("NotFoundError: {:?}", not_found_error);

    let validation_error = ValidationError {
        error: Some(Error {
            message: "Validation failed".to_string(),
            code: 400,
        }),
        fields: vec!["username".to_string(), "email".to_string()],
    };

    println!("ValidationError: {:?}", validation_error);

    let internal_error = InternalServerError {
        error: Some(Error {
            message: "Internal server error".to_string(),
            code: 500,
        }),
        details: "Null pointer exception".to_string(),
    };

    println!("InternalServerError: {:?}", internal_error);
}
