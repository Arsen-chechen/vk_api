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