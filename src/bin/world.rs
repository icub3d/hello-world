use tokio::signal::unix::{signal, SignalKind};
use tonic::{transport::Server, Request, Response, Status};
use triggered;

use hello_world_grpc::worlder_server::{Worlder, WorlderServer};
use hello_world_grpc::{WorldRequest, WorldResponse};

pub mod hello_world_grpc {
	tonic::include_proto!("hello_world_grpc");
}

#[derive(Debug, Default)]
pub struct MyWorlder {}

#[tonic::async_trait]
impl Worlder for MyWorlder {
	async fn get_world(
		&self,
		request: Request<WorldRequest>,
	) -> Result<Response<WorldResponse>, Status> {
		println!("World request: {:?}", request);
		Ok(Response::new(WorldResponse {
			world: Some(hello_world_grpc::World {
				world: hello_world::world(),
			}),
		}))
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = hello_world::addr().parse()?;
	let worlder = MyWorlder::default();

	let (trigger, trigger_signal) = triggered::trigger();
	let server_signal = trigger_signal.clone();
	let task = tokio::spawn(async move {
		Server::builder()
			.add_service(WorlderServer::new(worlder))
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
