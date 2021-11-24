use tokio::signal::unix::{signal, SignalKind};
use tonic::{transport::Channel, transport::Server, Request, Response, Status};
use triggered;

use hello_world_grpc::greeter_server::{Greeter, GreeterServer};
use hello_world_grpc::{helloer_client::HelloerClient, worlder_client::WorlderClient};
use hello_world_grpc::{GreeterRequest, GreeterResponse};
use hello_world_grpc::{HelloRequest, WorldRequest};

pub mod hello_world_grpc {
	tonic::include_proto!("hello_world_grpc");
}

#[derive(Debug)]
pub struct MyGreeter {
	helloer_client: HelloerClient<Channel>,
	worlder_client: WorlderClient<Channel>,
}

impl MyGreeter {
	async fn new(hello: String, world: String) -> Result<MyGreeter, Box<dyn std::error::Error>> {
		let helloer_client = HelloerClient::connect(hello).await?;
		let worlder_client = WorlderClient::connect(world).await?;

		Ok(MyGreeter {
			helloer_client,
			worlder_client,
		})
	}
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
	async fn get_greeting(
		&self,
		request: Request<GreeterRequest>,
	) -> Result<Response<GreeterResponse>, Status> {
		println!("Greet request: {:?}", request);

		let request = tonic::Request::new(HelloRequest {});
		let hello_response = self.helloer_client.clone().get_hello(request).await?;

		let request = tonic::Request::new(WorldRequest {});
		let world_response = self.worlder_client.clone().get_world(request).await?;

		Ok(Response::new(GreeterResponse {
			greeting: Some(hello_world_grpc::Greeting {
				greeting: hello_response.get_ref().hello.clone().unwrap().hello
					+ ", " + &world_response.get_ref().world.clone().unwrap().world
					+ ".",
			}),
		}))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = hello_world::addr().parse()?;
	let greeter =
		MyGreeter::new(hello_world::service("hello"), hello_world::service("world")).await?;

	let (trigger, trigger_signal) = triggered::trigger();
	let server_signal = trigger_signal.clone();
	let task = tokio::spawn(async move {
		Server::builder()
			.add_service(GreeterServer::new(greeter))
			.serve_with_shutdown(addr, server_signal)
			.await
	});

	let mut sigint = signal(SignalKind::interrupt())?;
	let mut sigterm = signal(SignalKind::terminate())?;
	tokio::select! {
		_ = sigint.recv() => {
			eprintln!("received interrupt");
		},
		_ = sigterm.recv() => {
			eprintln!("received terminate");
		},
	}

	trigger.trigger();
	let _ = task.await?;
	Ok(())
}
