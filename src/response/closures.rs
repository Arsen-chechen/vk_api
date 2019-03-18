/// Why this module was been created? Because:
///error[E0631]: type mismatch in function arguments
///  --> src\long_polling.rs:77:67
///   |
///77 |     ts:response.get("ts").ok_or(not_found("ts", "response")).and_then(Response::to_string)?,
///   |                                                              ^^^^^^^^ expected signature of `fn(response::Response) -> _`
///   |  
///::: src\response.rs:21:5
///   |
///21 |     pub fn to_string(&self) -> Result<String, Error> {
///   |     ------------------------------------------------ found signature of `for<'r> fn(&'r response::Response) -> _`

use crate::response::Response;
use crate::response::error::ResponseError as RE;

pub fn to_string(resp: Response) -> Result<String, RE> {resp.to_string()}

pub fn to_vec(resp: Response) -> Result<Vec<Response>, RE> {resp.to_vec()}

pub fn to_i64(resp: Response) -> Result<i64, RE> {resp.to_i64()}

pub fn to_f64(resp: Response) -> Result<f64, RE> {resp.to_f64()}

pub fn to_bool(resp: Response) -> Result<bool, RE> {resp.to_bool()}