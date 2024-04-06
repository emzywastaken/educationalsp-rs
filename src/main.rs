use std::io::{self, BufRead, BufReader, Read, Write};

use crate::lsp::initialize::InitializeRequest;

mod lsp;
mod rpc;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        log!("ERROR: {}\n\n\n", info);
    }));
    
    log!("INFO: Hey, I just started\n");

    let stdin = io::stdin().lock();
    
    let mut reader = BufReader::new(stdin);
    let mut content_length;
    let mut header_buf = String::new();
    loop {
        header_buf.clear();
        loop {
            if reader.read_line(&mut header_buf).unwrap() == 0 {
                log!("ERROR: No more bytes to read, exiting...\n");
                return;
            }
            let (header, _) = match header_buf.split_once("\r\n\r\n") {
                Some(v) => v,
                None => continue,
            };
            log!("INFO: header: {}\n", header);
    
            let content_length_bytes = &header["Content-Length: ".len()..header_buf.len() - 4];
            content_length = content_length_bytes.parse::<usize>().unwrap();
            break;
        }
    
        let mut content_buf = vec![0; content_length];
        reader.read_exact(&mut content_buf).unwrap();
        let msg = String::from_utf8_lossy(&content_buf);
    
        let resp = rpc::decode_message(&msg).unwrap();
        handle_message(&resp.method, &resp.content);
    }
}

fn handle_message(method: &str, contents: &str) {
    log!("INFO: Received msg with method: `{}`\n", method);
    // log!("INFO: contents: {}\n", contents);

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
            writer.write_all(reply.as_bytes()).unwrap();
            writer.flush().unwrap();
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

        let mut log_file = opts.open("/home/emzy/documents/dev/rust/poopi_doopi/lsp/lsp.log").unwrap();
        log_file.write_all($msg.as_bytes()).unwrap();
    }};
    ($msg:expr, $($arg:expr),+) => {{
            use std::fs;
            use std::io::Write;
            let mut opts = fs::OpenOptions::new();
            opts.append(true);
            opts.write(true);

            let mut log_file = opts.open("/home/emzy/documents/dev/rust/poopi_doopi/lsp/lsp.log").unwrap();
            let message = format!($msg, $($arg),+);
            log_file.write_all(message.as_bytes()).unwrap();
    }};
}
