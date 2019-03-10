use crate::{VK, par};
use crate::response::{Response, GettingFromResponseFor};

use std::error::Error;
use serde_json::Value;
/*
extern crate reqwest;
extern crate serde_json;
use std::boxed::Box;


use serde_json::value::Index;
use std::string::ToString;
extern crate serde;
use serde::de::DeserializeOwned;
use std::ops::Index as IndexTrait;
*/
type Handler = &'static Fn(Response, &VK);

trait Poll: Sized {
	fn polling(vk: &VK, handler: Handler) {
		Self::polling_with_wait(vk, 25, handler)
	}
	fn polling_with_wait(vk: &VK, wait: u8, handler: Handler) {
		let mut server = Self::get_long_poll_server(vk)
		.unwrap()
		.with_wait(wait);

		loop {
			for update in server.poll().unwrap() {
				handler(update, vk);
			}	
		}
	}

	fn poll(&mut self) -> Result<Vec<Response>, Box<Error>>;

	//builders
	fn get_long_poll_server(vk: &VK) -> Result<Self, Box<Error>>;

	#[allow(patterns_in_fns_without_body)]
	fn with_wait(mut self, wait: u8) -> Self;
}

#[derive(Debug)]
struct UserPolling {
	field: String
}

#[derive(Debug, Clone)]
pub struct GroupPolling {
	key: String,
	server: String,
	ts: String,
	wait: u8
}

impl Poll for GroupPolling {
	fn poll(&mut self) -> Result<Vec<Response>, Box<Error>> {
		let unk_err = "Unknown json from vk";
		let resp: Value = reqwest::get(
			format!("{server}?act=a_check&key={key}&ts={ts}&wait={w}", server=self.server, key=self.key, ts=self.ts, w=self.wait)
		.as_str())?
		.json()?;
		let response = Response(resp.clone());

		if resp["updates"]!=Value::Null {
			self.ts = response.get("ts")?;
			return Ok(response.get("updates")?)
		} else {
			return Err(From::from(unk_err))
		}
	}
	
	
	fn get_long_poll_server(vk: &VK) -> Result<GroupPolling, Box<Error>> {
		let resp = vk.call_gi("groups.getLongPollServer", par![])?;
		let response = Response(resp);
		let heh = response.g("heh");
		println!("{:#?}", heh);
		Ok(GroupPolling{
			key: response.get("key")?,
			server: response.get("server")?,
			ts: response.get("ts")?,
			wait: 25
		})
	}

	fn with_wait(mut self, wait: u8) -> Self {
		self.wait = wait;
		self
	}
}