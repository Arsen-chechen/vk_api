extern crate serde_json;
extern crate serde;
use serde_json::value::Index;
use serde_json::Value;
use crate::response::error::ResponseError as RE;
use serde_json::error::Error;
pub use serde_json::from_value as fv;

pub mod closures;
pub mod error;

#[derive(Clone, Debug)]
pub struct Response(pub Value);


impl Response {
	pub fn get<I>(&self, index: I) -> Result<Response, RE>
	where I: Index + Sized + ToString + Clone {
		let val = &self.0[index.clone()];
		if *val == Value::Null {
			return Err(RE::field_not_found(index.clone().to_string(), self.to_string()?))
		}
		Ok(Response(val.clone()))
	}
	pub fn to_string(&self) -> Result<String, RE> {
		let ret: Result<String, Error> = fv(self.0.clone());
		match ret {
			Err(ref e) if e.to_string().contains("invalid type") => Ok(self.0.clone().to_string()),
			_ => Ok(ret?)
		}
	}
	//may not deserialize
	pub fn to_vec(&self) -> Result<Vec<Response>, RE> {
		let temp: Vec<Value> = fv(self.0.clone())?;
		let mut ret: Vec<Response> = Vec::new();
		for t in temp {
			ret.push(Response(t));
		}
		Ok(ret)
	}
	pub fn to_i64(&self) -> Result<i64, RE> {
		Ok(fv(self.0.clone())?)
	}
	pub fn to_f64(&self) -> Result<f64, RE> {
		Ok(fv(self.0.clone())?)
	}
	pub fn to_bool(&self) -> Result<bool, RE> {
		Ok(fv(self.0.clone())?)
	}
}

pub trait AndThenGetting {
	fn and_get<I>(self, index: I) -> Result<Response, RE> where I: Index + Sized + ToString + Clone;
}
impl AndThenGetting for Result<Response, RE> {
	fn and_get<I>(self, index: I) -> Result<Response, RE>
	where I: Index + Sized + ToString + Clone{
		match self {
			Err(_) => self,
			Ok(heh) => heh.get(index)
		}
	}
}

#[cfg(test)]
	#[test]
	fn test_get() {
		let data = r#"
{
	"response": [{
		"id": 210700286,
		"first_name": "Lindsey",
		"last_name": "Stirling",
		"is_closed": false,
		"can_access_closed": true,
		"nickname": "",
		"city": {
			"id": 5331,
			"title": "Los Angeles"
		},
		"photo_max": "https://sun3-2.us...3qx9OtttE.jpg?ava=1",
		"has_mobile": 1,
		"can_write_private_message": 0,
		"mobile_phone": "",
		"home_phone": "",
		"verified": 1,
		"personal": {
			"religion": "Mormon",
			"inspired_by": "My Parents",
			"people_main": 6,
			"life_main": 5,
			"smoking": 1,
			"alcohol": 1
		}
	}]
}"#;

		let r: Option<Response> = Response(serde_json::from_str(data).unwrap()).get("response");
		let u: Option<Response> = r.and_get(0);
		assert_eq!(u.and_get("has_mobile").unwrap().to_i64().unwrap(), 1);
		assert_eq!(u.and_get("has_mobile").unwrap().to_string().unwrap(), "1");
		let personal: Response = u.and_get("personal").unwrap();
		assert_eq!(personal.get("religion").unwrap().to_string().unwrap(), "Mormon");
		assert_eq!(u.and_get("city").and_get("id").unwrap().to_string().unwrap(), "5331");
		assert!(u.and_get("heh").is_none());
		let city: &str = r#"{"id":5331,"title":"Los Angeles"}"#;
		assert_eq!(r.unwrap().get(0).and_get("city").unwrap().to_string().unwrap(), city);
	}
