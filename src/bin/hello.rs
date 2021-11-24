use tokio::signal::unix::{signal, SignalKind};
use tonic::{transport::Server, Request, Response, Status};
use triggered;

use hello_world_grpc::helloer_server::{Helloer, HelloerServer};
use hello_world_grpc::{HelloRequest, HelloResponse};

pub mod hello_world_grpc {
	tonic::include_proto!("hello_world_grpc");
}

#[derive(Debug, Default)]
pub struct MyHelloer {}

#[tonic::async_trait]
impl Helloer for MyHelloer {
	async fn get_hello(
		&self,
		request: Request<HelloRequest>,
	) -> Result<Response<HelloResponse>, Status> {
		println!("hello request: {:?}", request);
		Ok(Response::new(HelloResponse {
			hello: Some(hello_world_grpc::Hello {
				hello: hello_world::hello(),
			}),
		}))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get our listening address.
	let addr = hello_world::addr().parse()?;
	let helloer = MyHelloer::default();

	// Setup our trigger to signal the server to stop.
	let (trigger, trigger_signal) = triggered::trigger();
	let server_signal = trigger_signal.clone();

	// Spawn our server in another thread.
	let task = tokio::spawn(async move {
		Server::builder()
			.add_service(HelloerServer::new(helloer))
			.serve_with_shutdown(addr, server_signal)
			.await
	});

	// Listen for signals.
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

	// If we get here, we got a signal, notify the server.
	trigger.trigger();

	// Wait for the server to finish up.
	let _ = task.await?;
	Ok(())
}
