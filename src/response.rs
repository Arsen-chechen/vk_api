extern crate serde_json;
use serde_json::value::Index;
use serde_json::Value;
use std::error::Error;

pub trait GettingFromResponseFor<T> {
	fn get<I>(&self, index: I) -> Result<T, Box<Error>> where I: Index + Sized;
}

#[derive(Clone, Debug)]
pub struct Response(pub Value);
/*
*/

impl Response {
	pub fn g<I>(&self, index: I) -> Response
	where I: Index + Sized {
		Response(self.0[index].clone())
	}
}

impl GettingFromResponseFor<Response> for Response {
	fn get<I>(&self, index: I) -> Result<Response, Box<Error>> 
	where I: Index + Sized {
		Ok(self.g(index))
	}
}
impl GettingFromResponseFor<String> for Response {
	fn get<I>(&self, index: I) -> Result<String, Box<Error>>
	where I: Index + Sized {
		let temp: String = serde_json::from_value(self.g(index).0)?;
		Ok(temp)
	}
}
impl GettingFromResponseFor<Vec<Response>> for Response {
	fn get<I>(&self, index: I) -> Result<Vec<Response>, Box<Error>> 
	where I: Index + Sized {
		let temp: Vec<Value> = serde_json::from_value(self.g(index).0)?;
		let mut ret: Vec<Response> = Vec::new();
		for t in temp {
			ret.push(Response(t));
		}
		Ok(ret)
	}
}