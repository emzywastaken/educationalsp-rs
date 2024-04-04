use std::io::{self, BufRead, BufReader, Read, Write};

use crate::lsp::initialize::InitializeRequest;

mod lsp;
mod rpc;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        log!("ERROR: {}\n\n\n", info);
    }));
    ctrlc::set_handler(|| {
        log!("INFO: lsp stopped\n\n\n");
        std::process::exit(3);
    }).unwrap();
    
    log!("INFO: Hey, I just started\n");

    let stdin = io::stdin().lock();
    let mut reader = BufReader::new(stdin);

    let mut buf = String::new();
    let mut count = 0;
    loop {
        count += 1;
        log!("WARN: loop '{}\n", count);
        let _read = reader.read_line(&mut buf).unwrap();
        log!("INFO: current buf: `{:?}`\n", buf);

        match rpc::split(&buf) {
            Some(ttl_length) => {
                log!("INFO: ttl_lenght: {}\n", ttl_length);
                let lenght = buf.split_once("\r\n\r\n").unwrap();
                log!("INFO: header lenght: {}\n", lenght.0.len());
                let mut new_buf =vec![0; ttl_length - lenght.0.len() - 4];
                reader.read_exact(&mut new_buf).unwrap();
                let buf_str = String::from_utf8_lossy(&new_buf);
                buf.push_str(&buf_str);
                log!("INFO: buf len: {}\n", buf.len());
                log!("INFO: temp buf: {}\n", buf_str);
            },
            None => {log!("INFO: couldlnt split on current line continuing \n"); continue;},
        };

        let msg = buf.as_str();
        let resp = match rpc::decode_message(msg) {
            Ok(v) => v,
            Err(err) => {
                log!("ERROR: {}\n", err);
                break;
            },
        };

        handle_message(&resp.method, &resp.content);
        log!("INFO: message handled\n");

        buf.clear();
        break;
    }
}

fn handle_message(method: &str, contents: &str) {
    log!("INFO: Received msg with method: `{}`\n", method);
    log!("INFO: contents: {}\n", contents);

    match method {
        "initialize" => {
            let request: InitializeRequest = match serde_json::from_str(contents) {
                Ok(v) => v,
                Err(err) => { log!("ERROR: Hey we couldn't parse this: {}\n", err); return; },
            };

            let client_info = request.params.client_info.as_ref().unwrap();
            log!("INFO: connected to {} {}\n", client_info.name, client_info.version);

            // hey... let's reply
            let msg = lsp::InitializeResponse::new(request.request.id);
            let reply = rpc::encode_message(&msg).unwrap();

            let mut writer = io::stdout();
            writer.write(reply.as_bytes()).unwrap();
            log!("INFO: replied with: {}\n", reply);
        },
        _ => (),
    }
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        use std::fs;
        let mut opts = fs::OpenOptions::new();
        opts.append(true);
        opts.write(true);

        let mut log_file = opts.open("lsp.log").unwrap();
        log_file.write($msg.as_bytes()).unwrap();
    }};
    ($msg:expr, $($arg:expr),+) => {{
            use std::fs;
            use std::io::Write;
            let mut opts = fs::OpenOptions::new();
            opts.append(true);
            opts.write(true);

            let mut log_file = opts.open("lsp.log").unwrap();
            let message = format!($msg, $($arg),+);
            log_file.write(message.as_bytes()).unwrap();
    }};
}
