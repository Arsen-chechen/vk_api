extern crate reqwest;
extern crate serde_json;
use std::boxed::Box;
use std::error::Error;
use serde_json::Value;
extern crate rand;
use std::string::ToString;


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

/*макрос, написанный для десериализации в  serde_json, который позволяет 
в одну легкочитаемую строчку без повторения кода получать значения 
определённого(!) типа. Компилятор может выдать ошибку "Cannot infer type", 
если тип будет не определён. Примеры использования:
 get!(Value; key, key) //Так же как Value[key][key]

 let john = json!({
	"name": "John Doe",
	"age": 43,
	"phones": [
	    "+44 1234567",
		"+44 2345678"
	]
	});
	let new: String = get!(john; "phones", 0).unwrap();
	let old: String = serde_json::from_value(john["phones"][0].clone()).unwrap();
	assert_eq!(new, old);

Вызов без параметров:
	let rob: Value = get!(john;).unwrap();
	let first: String = serde_json::from_value(rob["phones"][0].clone()).unwrap();
	println!("{}", first);
	
*/
macro_rules! get {
	( $val:expr; $($x:expr),*) => (serde_json::from_value($val$([$x])*.clone()))
}



#[derive(Debug)]
pub struct DataOfServer {
	key: Box<str>,
	server: Box<str>,
	ts: Box<str>
}

impl DataOfServer {
	pub fn poll(&mut self) -> Result<Vec<Value>, Box<Error>> {
		let unk_err = "Unknown json from vk";
		let resp: Value = reqwest::get(
			format!("{server}?act=a_check&key={key}&ts={ts}&wait={w}", server=self.server, key=self.key, ts=self.ts, w=20)
		.as_str())?
		.json()?;

		if resp["updates"]!=Value::Null {
			self.ts = get!(resp; "ts")?;
			return Ok(get!(resp; "updates")?)
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
			key: get!(resp; "key")?,
			server: get!(resp; "server")?,
			ts: get!(resp; "ts")?
		})
	}
}