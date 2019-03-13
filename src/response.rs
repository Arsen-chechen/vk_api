extern crate serde_json;
use serde_json::value::Index;
use serde_json::Value;
use serde_json::Error;


#[derive(Clone, Debug)]
pub struct Response(pub Value);


impl Response {
	pub fn g<I>(&self, index: I) -> Response
	where I: Index + Sized {
		Response(self.0[index].clone())
	}
	pub fn get_string<I>(&self, index: I) -> Result<String, Error>
	where I: Index + Sized {
		serde_json::from_value(self.g(index).0)
	}
	//may not deserialize
	pub fn get_vec<I>(&self, index: I) -> Result<Vec<Response>, Box<Error>>
	where I: Index + Sized {
		let temp: Vec<Value> = serde_json::from_value(self.g(index).0)?;
		let mut ret: Vec<Response> = Vec::new();
		for t in temp {
			ret.push(Response(t));
		}
		Ok(ret)
	}
	pub fn get_i64<I>(&self, index: I) -> Result<i64, Error>
	where I: Index + Sized {
		serde_json::from_value(self.g(index).0)
	}
	pub fn get_f64<I>(&self, index: I) -> Result<f64, Error>
	where I: Index + Sized {
		serde_json::from_value(self.g(index).0)
	}
	pub fn get_bool<I>(&self, index: I) -> Result<bool, Error>
	where I: Index + Sized {
		serde_json::from_value(self.g(index).0)
	}
}

#[cfg(test)]
	#[test]
	fn test_g() {
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
		if r.g(0).get_i64("has_mobile").unwrap() == 1 {
			println!("Hello, mss's {} {}, i will call you at the number {}, yesss!", r.g(0).get_string("first_name").unwrap(), r.g(0).get_string("last_name").unwrap() , r.g(0).get_string("mobile_phone").unwrap());
		} else {
			panic!("...How did it happen...");
		}
		let religion = r.g(0).g("personal").get_string("religion").unwrap();
		if religion == "Pravoslavie"{
			panic!(format!("don't talk to {}", r.g(0).get_string("first_name").unwrap()));
		} else if religion == "Mormon" {
			println!("strange but ok");
		}

	}
