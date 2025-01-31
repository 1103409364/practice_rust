use axum::{extract::Path, http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/game/{guess}", get(guess))
        .route("/double/{number}", get(double));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// 等效于闭包 get(|Path(guess): Path<String>| async move { format!("The guess is {guess}") }),
async fn guess(Path(guess): Path<String>) -> String {
    format!("The guess is {guess} StatusCode {}", StatusCode::OK)
}

// use axum::{extract::Path, routing::get};
async fn double(Path(input): Path<String>) -> String {
    match input.parse::<i32>() {
        Ok(num) => format!("{} times 2 is {}!", num, num * 2),
        Err(e) => format!("Uh oh, weird input: {e}"),
    }
}
