use std::io::{self, BufRead, BufReader, Read, Write};
use std::panic::take_hook;

use crate::lsp::initialize::InitializeRequest;
use crate::lsp::text_document::did_change::TextDocumentDidChangeNotification;
use crate::lsp::text_document::did_open::DidOpenTextDocumentNotification;

mod lsp;
mod rpc;
mod analysis;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        log!("ERROR: {}\n\n\n", info);
        let default_hook = take_hook();
        default_hook(info)
    }));
    
    log!("INFO: Hey, I just started\n");

    let stdin = io::stdin().lock();
    
    let mut reader = BufReader::new(stdin);
    let mut content_length;
    let mut header_buf = String::new();
    let mut state = analysis::State::new();
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
            // log!("INFO: header: {}\n", header);
    
            let content_length_bytes = &header["Content-Length: ".len()..header_buf.len() - 4];
            content_length = content_length_bytes.parse::<usize>().unwrap();
            break;
        }
    
        let mut content_buf = vec![0; content_length];
        reader.read_exact(&mut content_buf).unwrap();
        let msg = String::from_utf8_lossy(&content_buf);
    
        let resp = rpc::decode_message(&msg).unwrap();
        handle_message(&mut state, &resp.method, &resp.content);
    }
}

fn handle_message(state: &mut analysis::State, method: &str, contents: &str) {
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
        "textDocument/didOpen" => {
            let request: DidOpenTextDocumentNotification = match serde_json::from_str(contents) {
                Ok(v) => v,
                Err(err) => { log!("ERROR: textDocument/didOpen: {}\n", err); return; },
            };

            log!("INFO: opened: {}\n", request.params.text_document.uri);
            state.open_document(request.params.text_document.uri, request.params.text_document.text)
        }
        "textDocument/didChange" => {
            let request: TextDocumentDidChangeNotification = match serde_json::from_str(contents) {
                Ok(v) => v,
                Err(err) => { log!("ERROR: textDocument/didOpen: {}\n", err); return; },
            };

            log!("INFO: changed: {}\n", request.params.text_document.identifier.uri);
            for change in request.params.content_changes {
                state.update_document(&request.params.text_document.identifier.uri, change.text);
            }
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
