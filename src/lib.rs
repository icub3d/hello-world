pub fn addr() -> String {
	let authority = match std::env::var("AUTHORITY") {
		Ok(authority) => authority,
		Err(_) => "0.0.0.0".to_string(),
	};
	let port = match std::env::var("PORT") {
		Ok(port) => port,
		Err(_) => "50051".to_string(),
	};
	format!("{}:{}", authority, port)
}

pub fn service(name: &str) -> String {
	let name = name.to_uppercase();
	let host = match std::env::var(format!("{}_SERVICE_HOST", name)) {
		Ok(host) => host,
		Err(_) => "0.0.0.0".to_string(),
	};
	let port = match std::env::var(format!("{}_SERVICE_PORT", name)) {
		Ok(port) => port,
		Err(_) => "50051".to_string(),
	};

	format!("http://{}:{}", host, port)
}

pub fn hello() -> String {
	"hello".to_string()
}

pub fn world() -> String {
	"world".to_string()
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
