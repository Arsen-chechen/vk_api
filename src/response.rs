extern crate serde_json;
use serde_json::value::Index;
use serde_json::Value;
use serde_json::Error;
pub use serde_json::from_value as fv;


#[derive(Clone, Debug)]
pub struct Response(pub Value);


impl Response {
	pub fn get<I>(&self, index: I) -> Option<Response>
	where I: Index + Sized {
		let val = &self.0[index];
		if *val == Value::Null {
			return None
		}
		Some(Response(val.clone()))
	}
	pub fn to_sring<I>(&self, index: I) -> Result<String, Error>
	where I: Index + Sized + Copy {
		let ret: Result<String, Error> = fv(self.0);
		match ret {
			Err(ref e) if e.to_string().contains("invalid type") => Ok(self.0.to_string()),
			_ => ret
		}
	}
	//may not deserialize
	pub fn to_vec<I>(&self, index: I) -> Result<Vec<Response>, Box<Error>>
	where I: Index + Sized {
		let temp: Vec<Value> = fv(self.g(index).unwrap().0)?;
		let mut ret: Vec<Response> = Vec::new();
		for t in temp {
			ret.push(Response(t));
		}
		Ok(ret)
	}
	pub fn to_i64<I>(&self, index: I) -> Result<i64, Error>
	where I: Index + Sized {
		fv(self.0)
	}
	pub fn to_f64<I>(&self, index: I) -> Result<f64, Error>
	where I: Index + Sized {
		fv(self.0)
	}
	pub fn to_bool<I>(&self, index: I) -> Result<bool, Error>
	where I: Index + Sized {
		fv(self.0)
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

		let r = Response(serde_json::from_str(data).unwrap()).g("response");
		let u = r.and_then(|i| i.g(0));
		assert_eq!(u.and_then(|i| i.g("has_mobile")).to_i64().unwrap(), 1);
		assert_eq!(u.gets("has_mobile").unwrap(), "1");
		let personal = u.g("personal").unwrap();
		assert_eq!(personal.gets("religion").unwrap(), "Mormon");
		assert_eq!(u.g("city").unwrap().gets("id").unwrap(), "5331");
		assert!(u.gets("heh").is_err());
		assert!(r.g());
	}
