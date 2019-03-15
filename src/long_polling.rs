use crate::{VK, par};
use crate::response::Response;

use std::error::Error;
use serde_json::Value;

use crate::response::closures::{to_string, to_vec, not_found};

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
struct UserPolling {
	heh: String
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
		let resp: Value = reqwest::get(
			format!("{server}?act=a_check&key={key}&ts={ts}&wait={w}", server=self.server, key=self.key, ts=self.ts, w=self.wait)
		.as_str())?
		.json()?;
		let response = Response(resp.clone());

		let ret = response.get("updates").ok_or(not_found("updates", "response from vk server")).and_then(to_vec)?;
		self.ts = response.get("ts").ok_or(not_found("ts", "response from vk server")).and_then(to_string)?;
		return Ok(ret)
	}
	
	//builders
	fn get_long_poll_server(vk: &VK) -> Result<GroupPolling, Box<Error>> {
		let response = vk.call_gi("groups.getLongPollServer", par![])?;
		let heh = response.get("heh");
		println!("{:#?}", heh);
		Ok(GroupPolling{
			key:	response.get("key")		.ok_or(not_found("key", "response"))	.and_then(to_string)?,
			server:	response.get("server")	.ok_or(not_found("server", "response"))	.and_then(to_string)?,
			ts: 	response.get("ts")		.ok_or(not_found("ts", "response")) 	.and_then(to_string)?,
			wait: 25
		})
	}

	fn with_wait(mut self, wait: u8) -> Self {
		self.wait = wait;
		self
	}
}