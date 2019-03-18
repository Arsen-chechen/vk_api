use std::error;
use std::fmt;
use std::convert::From;
extern crate serde_json;
extern crate reqwest;

#[derive(Debug)]
pub enum ResponseError {
	FieldNotFound(String),
	NoImportantFieldsFound(String),
	InvalidJson(String),
	ServerError(String),
	SerdeJsonError(serde_json::Error),
	ReqwestError(reqwest::Error)
}

impl ResponseError {
	pub fn field_not_found(field_name: String, json: String) -> Self {
		ResponseError::FieldNotFound(
			format!("field `{}` not found in json: {}", field_name, json)
		)
	}
	pub fn no_important_fields_found(json: String) -> Self {
		ResponseError::NoImportantFieldsFound(format!(r#"Something went wrong. 
VK returned data in which there was no important fields: {}"#, json)
		)
	}
	pub fn invalid_json(json: String) -> Self {
		ResponseError::InvalidJson(
			format!("Json is not valid and cannot be recognized. Json: {}", json)
		)
	}
	pub fn server_error(json: String) -> Self {
		ResponseError::ServerError(
			format!("The server returned an error. Error: {}", json)
		)
	}
}

impl fmt::Display for ResponseError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(std::error::Error::description(self))
	}
}

impl error::Error for ResponseError {
	fn description(&self) -> &str {
		match *self {
			ResponseError::FieldNotFound(ref msg) => msg,
			ResponseError::InvalidJson(ref msg) => msg,
			ResponseError::ServerError(ref msg) => msg,
			ResponseError::NoImportantFieldsFound(ref msg) => msg,
			ResponseError::SerdeJsonError(ref err) => err.description(),
			ResponseError::ReqwestError(ref err) => err.description(),
		}
	}
}

impl From<serde_json::Error> for ResponseError {
    fn from(error: serde_json::Error) -> Self {
        ResponseError::SerdeJsonError(error)
    }
}
impl From<reqwest::Error> for ResponseError {
	fn from(error: reqwest::Error) -> Self {
		ResponseError::ReqwestError(error)
	}
}


#[cfg(test)]
use ResponseError as RE;
use std::error::Error;
	#[test]
	pub fn test_field_not_found() {
		assert_eq!(RE::field_not_found("uh".to_string(), "heh".to_string()).description(),
		"field `uh` not found in json: heh"
		)
	}
	#[test]
	pub fn test_no_important_fields_found() {
		assert_eq!(RE::no_important_fields_found("Something".to_string()).description(),
		r#"Something went wrong. 
VK returned data in which there was no important fields: Something"#
		)
	}
	#[test]
	pub fn test_invalid_json() {
		assert_eq!(RE::invalid_json("Something".to_string()).description(),
		"Json is not valid and cannot be recognized. Json: Something"
		)
	}
	#[test]
	pub fn test_server_error() {
		assert_eq!(RE::server_error("Something".to_string()).description(),
		"The server returned an error. Error: Something"
		)
	}
