extern crate reqwest;
extern crate serde_json;
use std::boxed::Box;
use std::error::Error;
use serde_json::Value;
extern crate rand;

#[derive(Debug)]
pub struct DataOfServer {
	key: Box<str>,
	server: Box<str>,
	ts: Box<str>
}

impl DataOfServer {
	pub fn poll(&mut self) -> Result<Vec<Value>, Box<Error>> {
		let unk_err = "Unknown json from vk";
		let mut resp: Value = reqwest::get(
			format!("{server}?act=a_check&key={key}&ts={ts}&wait={w}", server=self.server, key=self.key, ts=self.ts, w=20)
		.as_str())?
		.json()?;

		if resp["updates"]!=Value::Null {
			self.ts = as_str_handle(&resp["ts"])?;
			return Ok(as_arr_handle(&mut resp["updates"])?)
		} else {
			return Err(From::from(unk_err))
		}
	}
}

#[derive(Debug)]
pub struct VkData {
	pub access_token: &'static str,
	pub version: &'static str,
	pub group_id: &'static str,
	pub url: &'static str
}

impl VkData {

	fn method(&self, method: &str, params: std::vec::Vec<(&str, &str)>) -> 
		Result<Value, Box<Error>> {
		let unk_err = "Unknown json from vk";

		let mut text = String::new();
		for p in params.iter() {
			text = text + p.0 + "=" + p.1 + "&";
		}
		text += &format!("access_token={AT}&v={V}", AT=&self.access_token, V=&self.version);

		let data:Value = reqwest::get(
			format!("{Url}{Method}?{Params}", Url=&self.url, Method=method, Params=text)
		.as_str())?
		.json()?;

		if data["response"]!=Value::Null {
			return Ok(data["response"].clone())
		} else if data["error"]!=Value::Null {
			return Err(From::from(format!("Error: {}", data["error"])))
		} else {
			return Err(From::from(unk_err))
		}
	}

	#[allow(non_snake_case)]
	pub fn messages_getHistory(&self, user_id: i64) -> Result<Value, Box<Error>> {
		Ok( self.method("messages.getHistory", 
			vec![("count", "1"), ("user_id", &user_id.to_string()), ("group_id", self.group_id)]
		)? )
	}

	pub fn messages_send(&self, message: String, user_id: i64) -> Result<(), Box<Error>> {
		let x = rand::random::<i32>();
		self.method("messages.send",
			vec![("user_id", &user_id.to_string()), ("random_id", &x.to_string()), ("message", &message), ("dont_parse_link", "0")]
		)?;
		Ok(())
	}

	pub fn users_get(&self, user_id: i64) -> Result<Value, Box<Error>> {
		Ok( self.method("users.get",
			vec![("user_ids", &user_id.to_string()), ("name_case", "Nom")]
		)? )
	}

	#[allow(non_snake_case)]
	pub fn groups_GetLongPollServer(&self) -> Result<DataOfServer, Box<Error>> {
		let resp = self.method("groups.getLongPollServer",
			vec![("group_id", self.group_id)]
		)?;
		Ok(DataOfServer{
			key: as_str_handle(&resp["key"])?,
			server: as_str_handle(&resp["server"])?,
			ts: as_str_handle(&resp["ts"])?
		})
	}
}

pub fn as_str_handle(s: &Value) -> Result<Box<str>, Box<Error>> {
	Ok(s.as_str().ok_or("Unknown json from vk")?.into())
}

pub fn as_i64_handle(i: &Value) -> Result<i64, Box<Error>> {
	let unk_err = "Unknown json from vk";
	Ok(i.as_i64().ok_or(unk_err)?)
}

pub fn as_arr_handle(a: &mut Value) -> Result<Vec<Value>, Box<Error>> {
	let unk_err = "Unknown json from vk";
	Ok(a.as_array_mut().ok_or(unk_err)?.clone())
}
