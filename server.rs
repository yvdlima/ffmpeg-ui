#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rouille = "3.5.0"
//! ```

extern crate rouille;

use std::fs::File;
use rouille::Response;

fn main() {

    println!("Now listening on localhost:8080");

    rouille::start_server("localhost:8080", move |request| {
        {
            let response = rouille::match_assets(&request, "./dist/");

            if response.is_success() {
                return response;
            }
        }
        
        let file = File::open("dist/index.html").unwrap();
        let response = Response::from_file("text/html", file);

        Response::with_additional_header(response, "Cross-Origin-Embedder-Policy", "require-corp")
            .with_additional_header("Cross-Origin-Opener-Policy", "same-origin")
    });
}
