trait Poll {
	fn polling(handler: Fn) {
		polling_with_wait(25, handler)
	}
	fn polling_with_wait(wait: u8, handler: Fn) {
		let server = get_long_poll_server()
		.with_wait(wait);

		loop {
			for update in server_data.poll().unwrap() {
				handler(update, vk).unwrap();
			}	
		}
	}

	fn poll(&mut self) -> Result<Vec<Value>, Box<Error>>;

	//builders
	fn get_long_poll_server(vk: VK) -> Self;

	fn with_wait(mut self, wait: u8) -> Self {
		self.wait = wait;
		self
	}
}

#[derive(Debug)]
struct UserPolling {
	field: Type
}

#[derive(Debug)]
struct GroupPolling {
	key: String,
	server: String,
	ts: String,
	wait: u8
}

impl Poll for GroupPolling {
	pub fn poll(&mut self) -> Result<Vec<Value>, Box<Error>> {
		let unk_err = "Unknown json from vk";
		let resp: Value = reqwest::get(
			format!("{server}?act=a_check&key={key}&ts={ts}&wait={w}", server=self.server, key=self.key, ts=self.ts, w=self.wait)
		.as_str())?
		.json()?;

		if resp["updates"]!=Value::Null {
			self.ts = get!(resp; "ts")?;
			return Ok(get!(resp; "updates")?)
		} else {
			return Err(From::from(unk_err))
		}
	}
	
	
	pub fn get_long_poll_server(vk: VK) -> Result<DataOfServer, Box<Error>> {
		let resp = vk.call_gi("groups.getLongPollServer", par![])?;
		Ok(DataOfServer{
			key: get!(resp; "key")?,
			server: get!(resp; "server")?,
			ts: get!(resp; "ts")?,
			wait: 25
		})
	}
}