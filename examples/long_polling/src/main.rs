extern crate vk_api;
use vk_api::{VkData, DataOfServer, get, par, PutInAStrings};
use std::boxed::Box;
use std::error::Error;
extern crate serde_json;
use serde_json::Value;
extern crate rand;



fn main() {
	let vk = VkData::new("TOKEN")
	.set_group_id("ID YOUR GROUP(WHOSE TOKEN)");
	println!("{}", vk.call("users.get", par![("user_ids", 1), ("name_case", "Nom")]).unwrap().to_string());

	let mut server_data: DataOfServer = vk.groups_GetLongPollServer().unwrap();
	loop {
		let updates = server_data.poll_group(20).unwrap();
		for update in updates {
			handler(update, &vk).unwrap();
		}	
	}

}

fn handler(update: Value, vk: &VkData) -> Result<(), Box<Error>> {
	if update["type"] == "message_new" {
		let obj: Value = get!(update; "object")?;
		let user_id: i64 = get!(obj; "from_id")?;
		let users: Value = vk.call("users.get", par![("user_ids", user_id), ("name_case", "Nom")])?;
		let user: Value = get!(users; 0)? ;
		
		let first_name: String = get!(user; "first_name")?;
		let last_name: String = get!(user; "last_name")?;
		let user_text: String = get!(obj; "text")?;
		println!("{} {} Написал: {}", first_name, last_name, user_text);
		let history = vk.call_gi("messages.getHistory", par![("count", "1"), ("user_id", user_id)])?;
		let count: i64 = get!(history; "count")?;
		vk.call("messages.send", par![("user_id", user_id), ("random_id", rand::random::<i32>()), ("message", format!("Привет! Ты отправил {}-е сообщение!", count)) ])?;
	}
	Ok(())
}
