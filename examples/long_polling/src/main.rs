extern crate vk_api;
use vk_api::{VK, long_polling, response, par, PutInAStrings};
use long_polling::{GroupPolling};
use response::Response;
use std::boxed::Box;
use std::error::Error;
extern crate rand;



fn main() {
	loop {
		match raisable_function() {
			Ok(_) => return,
			Err(e) => {
				println!("{}", e.to_string());
				continue
			}
		}
	}	
}

fn raisable_function() -> Result<(), Error> {
	let vk = VkData:from_token("TOKEN")
	.with_group_id(123);
	println!("{}", 
		vk.call("users.get",
			par![("user_ids", 1),
				("name_case", "Nom")])
		.and_then(to_string)?
	);

	GroupPolling::poll(vk, handler)
}

fn handler(vk: &VK, update: Response) -> Result<(), Box<Error>> {
	let type_message = update.get("type")
		.ok_or(unknown_error("type", "update"))
		and_then(to_string)?;
	if type_message == "message_new" {
		let obj: Response = update.get("object")
			.ok_or(unknown_error("object", "update"));
		let user_id: i64 = obj.get("from_id")
			.ok_or(unknown_error("from id", "object"))
			.and_then(to_string)?;
		let users: Response = vk.call("users.get", par![("user_ids", user_id), ("name_case", "Nom")])?;
		let user: Response = users.get(0).ok_or(unknown_error("0", "users"))?;
		
		let first_name: String = user.get("first_name")
			.ok_or(unknown_error("first name", "user"))
			and_then(to_string)?;
		let last_name: String = user.get("last_name")
			.ok_or(unknown_error("last name", "user"))
			and_then(to_string)?;
		let user_text: String = obj.get("text")
			.ok_or(unknown_error("text", "obj"))
			and_then(to_string)?;
		println!("{} {} Написал: {}", first_name, last_name, user_text);
		let history = vk.call_gi("messages.getHistory", par![("count", "1"), ("user_id", user_id)])?;
		let count: i64 = history.get("count").ok_or(unknown_error("count", "history from server"))?;
		vk.call("messages.send", par![("user_id", user_id), ("random_id", rand::random::<i32>()), ("message", format!("Привет! Ты отправил {}-е сообщение!", count)) ])?;
	}
	Ok(())
}
