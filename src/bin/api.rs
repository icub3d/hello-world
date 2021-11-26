#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status::Custom;

use hello_world_grpc::greeter_client::GreeterClient;
use hello_world_grpc::GreeterRequest;

pub mod hello_world_grpc {
	tonic::include_proto!("hello_world_grpc");
}

async fn get_greeting() -> Result<String, Box<dyn std::error::Error>> {
	let greeting = hello_world::service("greeting");
	let mut client = GreeterClient::connect(greeting).await?;
	let request = tonic::Request::new(GreeterRequest {});
	let response = client.get_greeting(request).await?;
	Ok(response.get_ref().greeting.clone().unwrap().greeting)
}

#[get("/hello-world")]
async fn get_hello_world() -> Result<String, Custom<String>> {
	get_greeting()
		.await
		.map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/api", routes![get_hello_world])
}
