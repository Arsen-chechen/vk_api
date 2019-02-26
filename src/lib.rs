extern crate reqwest;
extern crate serde_json;
use std::boxed::Box;
use std::error::Error;
use serde_json::Value;
extern crate rand;


/*
Черта была создана для того, чтобы удобно создавать Vec<(String, String)>.
функция heh.put(1, 2) это удобная замена heh.push((1.to_string(), 2.to_string))
Необходимость в этом методе обусловлена чрезмерным повторением кода. 
Эта черта используется мной для того, чтобы отдавать функции параметры для web-api 
вида key=value. Поэтому все значение преобразуются в string, тип удобный для web.
*/
trait PutInAStrings {
	fn put<P, V>(&mut self, key: P, val: V)
	where P:ToString, V:ToString;
}
impl PutInAStrings for Vec<(String, String)> {
	fn put<P, V>(&mut self, key: P, val: V)
	where P:ToString, V:ToString {
		self.push((key.to_string(), val.to_string()));
	}
}
macro_rules! par {
	( $(($k:expr, $v:expr)),* ) => {
		{
			let mut temp: std::vec::Vec<(String, String)> = std::vec::Vec::new();
			$(
				temp.put($k, $v);
			)*
			temp
		}
	};
}
/* Как это работает:
	Есть разные пути создания Vec<(String, String)>, являющегося вектором кортежей, состоящих из двух строк.
	Этот тип переменной используется мной для передачи параметров в VkData.method().
	Стандартный путь это:
	let params = vec![("heh".to_string(), 1.1.to_string()), ("fuzz".to_string(), 5.to_string())];
	
	В нём используется слишком много to_string()
	я написал функцию put, которая делает то же самое что и Vec.push, 
	но при этом автоматически преобразует все не-String в String 
	(и принимает в качестве параметров аргументы вместо кортежа, см. пример) 
	let params1 = Vec::new();
	params1.put("id", 1234);
	params1.put("text", "hello");
	params1.put(5, 3.14);

	Но постойте, ведь раньше можно было напсать параметры хотя бы в одну строку 
	(пусть с повторяющимся кодом, но всё же), а теперь придётся писать так много строк?!
	Конечно же нет, ведь я написал и  (сильно) упрощённую версию vec!, которую назвал par!
	и с помощью неё можно снова писать параметры в одну строку:
	let params2 = par![("id", 1234.5), ("text", "privet"), ("dest_id", 1337)];

	Если после создания вектора вам захочется добавить ещё один параметр, 
	вы можете использовать put (но только с mut кортежами):
	let mut params3 = par![];
	params3.put("heh", "mda".to_string); //Да, String тоже можно передавать


*/

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

	fn method(&self, method: &str, parameters: std::vec::Vec<(String, String)>) -> 
		Result<Value, Box<Error>> {
		let unk_err = "Unknown json from vk";

		let mut params = String::new();
		for p in parameters.iter() {
			params = params + &p.0 + "=" + &p.1 + "&";
		}
		params += &format!("access_token={AT}&v={V}", AT=&self.access_token, V=&self.version);

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
		Ok( self.method("messages.getHistory", 
			par![("count", "1"), ("user_id", user_id), ("group_id", self.group_id)]
		)? )
	}

	pub fn messages_send(&self, message: String, user_id: i64) -> Result<(), Box<Error>> {
		self.method("messages.send",
			par![("user_id", user_id), ("random_id", rand::random::<i32>()), ("message", message), ("dont_parse_link", "0")]
		)?;
		Ok(())
	}

	pub fn users_get(&self, user_id: i64) -> Result<Value, Box<Error>> {
		Ok( self.method("users.get",
			par![("user_ids", user_id), ("name_case", "Nom")]
		)? )
	}

	#[allow(non_snake_case)]
	pub fn groups_GetLongPollServer(&self) -> Result<DataOfServer, Box<Error>> {
		let resp = self.method("groups.getLongPollServer",
			par![("group_id", self.group_id)]
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
