use signal_hook::{consts::signal::*, iterator::Signals};
use std::env;
use std::error::Error;
use std::io::Cursor;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let sigs = [SIGHUP, SIGTERM, SIGINT, SIGQUIT];
    let mut signals = Signals::new(&sigs)?;

    thread::spawn(move || {
        if let Some(s) = signals.into_iter().next() {
            println!("Caught signal {:?}; quitting", s);
            std::process::exit(0);
        }
    });

    let start = chrono::Utc::now();

    let colour = env::var("COLOUR").expect("Please supply a colour");
    println!("Starting with colour {}", colour);

    let html = format!(
        "\
<html>
    <head>
        <title>{c}</title>
    </head>
    <body style=\"background-color: {c}; color: white\">
        <h1>{c}</h1>
    <body>
<html>",
        c = colour
    );
    let json = format!(r#"{{ "colour": "{c}" }}"#, c = colour);

    let server = tiny_http::Server::http("0.0.0.0:8080").expect("Can't start server"); // TODO add logging library and do JSON / text
    println!("[{}] Listening on {:?}", colour, server.server_addr());

    for request in server.incoming_requests() {
        println!("[{}] {:?}", colour, request);

        if request.url().starts_with("/ready") && (chrono::Utc::now() - start).num_seconds() < 5 {
            let response = tiny_http::Response::empty(tiny_http::StatusCode(500));
            let _ = request.respond(response);
        } else {
            let (r, t) = if request.url().starts_with("/api") {
                (json.clone(), "Content-Type: application/json")
            } else {
                (html.clone(), "Content-Type: text/html; charset=UTF-8")
                /* NB: this will also reply 200 to any other path, including /live or whatever */
                // TODO: handle /healthz and /readyz, return json with status ok, color foo
                // TODO change to US eng everywhere
                // TODO read color as args[1]
                // TODO parse flags: --port, --color
            };
            let l = Some(r.len());
            /* There is a ::from_string() but that sets a `Content-Type: text/plain` which you
             * can't get rid of. */
            let response = tiny_http::Response::empty(tiny_http::StatusCode(200))
                .with_header(t.parse::<tiny_http::Header>().unwrap())
                .with_data(Cursor::new(r.into_bytes()), l);
            let _ = request.respond(response);
        }
    }

    Ok(())
}
