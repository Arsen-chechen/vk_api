use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ResponseError {
	FieldNotFound(String, String),
	NoImportantFieldsFound(String),
	InvalidJson(String),
	ServerError(String),
	
}

impl fmt::Display for ResponseError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(std::error::Error::description(self))
	}
}

impl error::Error for ResponseError {
	fn description(&self) -> &str {
		match *self {
			ResponseError::FieldNotFound(ref field, ref json) => &format!("field `{}` not found in json: {}", field, json),
			ResponseError::InvalidJson(ref json) => &format!("Json is not valid and cannot be recognized. Json: {}",json),
			ResponseError::ServerError(ref json) => &format!("The server returned an error. Error: {}", json),
			ResponseError::NoImportantFieldsFound(ref json) => &format!(r#"Something went wrong. 
				VK returned data in which there was no response or error: {}"#, json),
		}
	}
}