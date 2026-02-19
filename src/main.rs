fn main() {
    let input: Input = std::env::args().into();

    let client = reqwest::blocking::Client::new();

    let request = match (input.request_type, input.body) {
        (RequestType::Get, _) => client.get(input.url),
        (RequestType::Post, Some(body)) => client.post(input.url).body(body),
        (RequestType::Post, None) => client.post(input.url),
    };

    let mut response = request.send().unwrap();

    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

enum RequestType {
    Get,
    Post,
}

impl From<String> for RequestType {
    fn from(arg: String) -> RequestType {
        if arg == "post" {
            return RequestType::Post;
        }
        return RequestType::Get;
    }
}

struct Input {
    request_type: RequestType,
    url: String,
    body: Option<String>,
}

impl From<std::env::Args> for Input {
    fn from(mut args: std::env::Args) -> Self {
        let _program_path = args.next();
        let request_type = args.next().unwrap().into();
        let url = args.next().unwrap();
        let body = args.next();
        Input {
            request_type,
            url,
            body,
        }
    }
}