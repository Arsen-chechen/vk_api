extern crate vk_api;
use vk_api::VK;
use vk_api::update;
use vk_api::group;

fn main() {
	let group_id = 1234;
	let vk = VK::by_group("token", group_id);
	let users: update = call("users.get", ["user_ids": 1]).unwrap();
	let durov: update = update.get(0);
	let last_name: String = update.get("last_name");
	println!("{}, Верни стену!!", last_name);
	let second_name: String = update.getp(0, "last_name");
	assert_eq!(last_name, second_name);

	vk.polling(handler);
}

fn handler(vk: VK, heh: update) {
	let obj = update.get("object");
	if update.get("type") == "message_new" {
		let user_id: String = obj.get("from_id");
		let user = vk.call("users.get", ["user_ids": user_id], {"name_case": "Nom"})
		.unwrap()
		.get(0);
		let username: String = user.get("first_name");
		vk.call_gi("message.send", ["user_id", user_id], ["text", format!("{}, капец тебе! Я только что узнал твой адрес. Жди.", username)]);
	}
}