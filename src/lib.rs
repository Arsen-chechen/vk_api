extern crate reqwest;
extern crate serde_json;
use std::boxed::Box;
use std::error::Error;
use serde_json::Value;
use serde_json::value::Index;
use std::string::ToString;
extern crate serde;
use serde::de::DeserializeOwned;
use std::ops::Index as IndexTrait;

pub mod long_polling;
pub mod response;
pub use response::{Response, GettingFromResponseFor}; 

/*
Черта была создана для того, чтобы удобно создавать Vec<(String, String)>.
функция heh.put(1, 2) это удобная замена heh.push((1.to_string(), 2.to_string))
Необходимость в этом методе обусловлена чрезмерным повторением кода. 
Эта черта используется мной для того, чтобы отдавать функции параметры для web-api 
вида key=value. Поэтому все значение преобразуются в string, тип удобный для web.
*/
pub trait PutInAStrings {
	fn put<P, V>(&mut self, key: P, val: V)
	where P:ToString, V:ToString;
}
impl PutInAStrings for Vec<(String, String)> {
	fn put<P, V>(&mut self, key: P, val: V)
	where P:ToString, V:ToString {
		self.push((key.to_string(), val.to_string()));
	}
}
#[macro_export]
macro_rules! par {
	( $(($k:expr, $v:expr)),* ) => {
		{	#[allow(unused_mut)]
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

примечание: В случае с сохранением значения типа &Value, возможно, будет удобнее писать
	&Value[key];	
*/
/*
#[macro_export]
macro_rules! get {
	( $val:expr; $($x:expr),*) => (serde_json::from_value($val$([$x])*.clone()))
}
*/



// можно назвать эту структуру "клиентом vk-api"
#[derive(Debug, Clone)]
pub struct VK {
	pub access_token: String,
	pub version: f32,
	pub group_id: i64,
	pub client: reqwest::Client
}

impl VK {

	//
	pub fn call(&self, method: &str, mut parameters: std::vec::Vec<(String, String)>) -> 
		Result<Value, Box<Error>> {
		let unk_err = "Unknown json from vk";

		parameters.put("access_token", &self.access_token);
		parameters.put("v", &self.version);

		let data: Value = self.client.get(&format!("https://api.vk.com/method/{}", method))
    	.query(&parameters)
    	.send()?
		.json()?;

		if data["response"]!=Value::Null {
			return Ok(data["response"].clone())
		} else if data["error"]!=Value::Null {
			return Err(From::from(format!("Error: {}", data["error"])))
		} else {
			return Err(From::from(unk_err))
		}
	}
	//call with group_id parameter
	pub fn call_gi(&self, method: &str, mut parameters: std::vec::Vec<(String, String)>) -> 
		Result<Value, Box<Error>> {
		parameters.put("group_id", &self.group_id);
		self.call(method, parameters)
	}
	//builders
	pub fn from_token(token: String) -> Self {
		VK {
			access_token: token,
			version: 5.92,
			group_id: 0,
			client: reqwest::Client::new()
		}
	}
	#[allow(dead_code, unused_variables)]
	fn from_login(login: String, password: String) -> Self {
		let token = String::new();
		VK::from_token(token)
	}
	pub fn with_group_id(mut self, group_id: i64) -> Self {
		self.group_id = group_id;
		self
	}
}