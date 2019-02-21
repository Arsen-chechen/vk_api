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

	fn get(&self, method: &str, params: String) -> Result<Value, Box<Error>> {
		let unk_err = "Unknown json from vk";
		let data:Value = reqwest::get(
			format!("{Url}{Method}?{Params}", Url=&self.url, Method=method, Params=params)
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
		Ok( self.get("messages.getHistory", 
			format!("count={C}&user_id={UI}&group_id={GI}&access_token={AT}&v={V}",
				C=1, UI=user_id, GI=self.group_id, AT=self.access_token, V=self.version
			)
		)? )
	}

	pub fn messages_send(&self, message: String, user_id: i64) -> Result<(), Box<Error>> {
		let x = rand::random::<i32>();
		self.get("messages.send",
			format!("user_id={UI}&random_id={RI}&message={M}&dont_parse_link={DPL}&access_token={AT}&v={V}",
				UI=user_id, RI=x, M=message, DPL=0, AT=self.access_token, V=self.version
			)
		)?;
		Ok(())
	}

	pub fn users_get(&self, user_id: i64) -> Result<Value, Box<Error>> {
		Ok( self.get("users.get",
			format!("user_ids={UI}&name_case=Nom&access_token={AT}&v={V}",
				UI=user_id, AT=self.access_token, V=self.version
			)
		)? )
	}

	#[allow(non_snake_case)]
	pub fn groups_GetLongPollServer(&self) -> Result<DataOfServer, Box<Error>> {
		let resp = self.get("groups.getLongPollServer",
			format!("group_id={GI}&access_token={AT}&v={V}",
				GI=self.group_id, AT=self.access_token, V=self.version
			)
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
