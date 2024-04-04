#![allow(unused)]
use std::fmt::Display;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Write};
use std::os::fd::AsRawFd;
use std::{error::Error, fmt::Debug, fs};

use serde::{Deserialize, Serialize};

use crate::lsp::initialize::InitializeRequest;

mod lsp;
mod rpc;
fn main() {
    std::panic::set_hook(Box::new(|info| {
        log!(fmt: "error: {}\n\n\n", info);
    }));
    ctrlc::set_handler(|| {
        log!("info: lsp stopped\n\n\n");
        std::process::exit(3);
    });

    log!("info: Hey, I just started\n");

    let mut stdin = io::stdin().lock();
    let mut reader = BufReader::new(stdin);
    loop {
        // log!("loop start\n");
        // initial msg size is 3112
        let mut buf = [0; 3112];
        // log!("loop cont 1\n");
        let read = match reader.read(&mut buf) {
            Ok(v) => v,
            Err(err) => break,
        };
        // log!("loop cont 2");
        // eprintln!("loop count 2");
        let read_buf = String::from_utf8_lossy(&buf[..read]);
        // log!("loop cont 3\n");

        let msg = match rpc::split(&read_buf) {
            Some(v) => v,
            None => continue,
        };
        let resp = match rpc::decode_message(msg) {
            Ok(v) => v,
            Err(err) => {
                log!(fmt: "error: {}", err);
                continue;
            }
        };
        handle_message(&resp.method, &resp.content);
        log!("handles msg\n");
    }
}

fn handle_message(method: &str, contents: &str) {
    log!(fmt: "info: Received msg with method: `{}`\n", method);
    log!(fmt: "info: contents: {}\n", contents);

    match method {
        "initialize" => {
            let request: InitializeRequest = match serde_json::from_str(contents) {
                Ok(v) => v,
                Err(err) => { log!(fmt: "error: Hey we couldn't parse this: {}\n", err); return; },
            };

            let client_info = request.params.client_info.as_ref().unwrap();
            log!(fmt: "info: connected to {} {}\n", client_info.name, client_info.version);

            // hey... let's reply
            let msg = lsp::InitializeResponse::new(request.request.id);
            let reply = rpc::encode_message(&msg).unwrap();

            let mut writer = io::stdout();
            writer.write(reply.as_bytes());
            log!(fmt: "info: replied with: {}\n", reply);
        },
        _ => (),
    }
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        let mut opts = fs::OpenOptions::new();
        opts.append(true);
        opts.write(true);

        let mut log_file = opts.open("log.log").unwrap();
        log_file.write($msg.as_bytes());
    }};
    (fmt: $msg:expr, $($arg:expr),+) => {{
            let mut opts = fs::OpenOptions::new();
            opts.append(true);
            opts.write(true);

            let mut log_file = opts.open("log.log").unwrap();
            let message = format!($msg, $($arg),+);
            log_file.write(message.as_bytes());
    }};
}
