extern crate vk_api;
use vk_api::{VK, long_polling, response, par, PutInAString};
use long_polling::{GroupPolling, Poll};
use response::Response;
use response::error::ResponseError as Error;
use response::closures::{to_string, to_i64};
extern crate rand;


fn main() {
	use std::error::Error as Errr;
	loop {
		match raisable_function() {
			Ok(_) => return,
			Err(e) => {
				println!("{}", e);
				continue
			}
		}
	}	
}

fn raisable_function() -> Result<(), Error> {
	let vk = VK::from_token("token".to_string())
	.with_group_id(12345567890000);
	//testing
	println!("{}", 
		vk.call("users.get",
			par![("user_ids", 1),
				("name_case", "Nom")])
		.and_then(to_string)?
	);

	Ok(GroupPolling::polling(&vk, &handler))
}

//TODO: handler() -> Result<(), ResponseError>
fn handler(update: Response, vk: &VK) {
	let type_message = update.get("type")
		.and_then(to_string).unwrap();
	if type_message == "message_new" {
		let obj: Response = update.get("object").unwrap();
		let user_id: i64 = obj.get("from_id")
			.and_then(to_i64).unwrap();
		let users: Response = vk.call("users.get", par![("user_ids", user_id), ("name_case", "Nom")]).unwrap();
		let user: Response = users.get(0).unwrap();
		
		let first_name: String = user.get("first_name")
			.and_then(to_string).unwrap();
		let last_name: String = user.get("last_name")
			.and_then(to_string).unwrap();
		let user_text: String = obj.get("text")
			.and_then(to_string).unwrap();
		println!("{} {} Написал: {}", first_name, last_name, user_text);
		let history = vk.call_gi("messages.getHistory", par![("count", "1"), ("user_id", user_id)]).unwrap();
		let count: i64 = history.get("count").
			and_then(to_i64).unwrap();
		vk.call("messages.send", par![("user_id", user_id), ("random_id", rand::random::<i32>()),
			("message", format!("Привет! Ты отправил {}-е сообщение!", count)) ]).unwrap();
	}
}
