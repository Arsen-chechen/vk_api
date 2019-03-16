extern crate vk_api;
use vk_api::{VK, par, PutInAString};
use vk_api::response::{Response, AndThenGetting};
use vk_api::long_polling::{GroupPolling, Poll};

use vk_api::response::closures::{not_found, to_string};

fn main() {
	let group_id = 1234;
	let vk = VK::from_token("token".to_owned())
		.with_group_id(group_id);
	let users: Response = vk.call("users.get", par!(("user_ids", 1)))
		.expect("calling server raise error");
	let durov: Response = users.get(0)
		.expect("server do not return users");
	let last_name: String = durov.get("last_name")
		.expect("hah, durov haven't last name")
		.to_string()
		.expect("vk return invalid json");
	println!("{}, Верни стену!!", last_name);
	let second_name: String = users.get(0)
		.and_get("last_name")
		.ok_or(not_found("0 or last_name", "users"))
		.and_then(to_string)
		.unwrap();

	assert_eq!(last_name, second_name);

	GroupPolling::polling(vk, handler);
}

fn handler(vk: VK, update: Response) {
	let obj = update.get("object")
		.expect("object not found in the update");
	let update_type = update.get("type")
		.expect("there is no field `type` in the object")
		.to_string()
		.expect("vk return not valid json");
	if update_type == "message_new" {
		let user_id: String = obj.get("from_id").unwrap().to_string().unwrap();
		let user = vk.call("users.get", par![("user_ids", user_id), ("name_case", "Nom")])
			.unwrap()
			.get(0);
		let username: String = user.and_get("first_name")
			.expect("when calling the `users.get` some field was not found")
			.to_string()
			.expect("when calling the `users.get` something went wrong");
		vk.call_gi("message.send", par![("user_id", user_id), ("text", format!("{}, капец тебе! Я только что узнал твой адрес. Жди.", username))])
			.unwrap();
	}
}