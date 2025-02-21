use std::fs::OpenOptions;
use std::io::Write;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/tim/Documents/temp/log.txt")
            .unwrap();
        file.write_all(format!("initialize: {params:?}").into_bytes().as_slice())
            .unwrap();
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/tim/Documents/temp/log.txt")
            .unwrap();
        file.write_all(format!("initialized: {params:?}").into_bytes().as_slice())
            .unwrap();

        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/tim/Documents/temp/log.txt")
            .unwrap();
        file.write_all(format!("did_open: {params:?}").into_bytes().as_slice())
            .unwrap();
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/tim/Documents/temp/log.txt")
            .unwrap();
        file.write_all(format!("did_close: {params:?}").into_bytes().as_slice())
            .unwrap();
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("/home/tim/Documents/temp/log.txt")
            .unwrap();
        file.write_all(format!("did_change: {params:?}").into_bytes().as_slice())
            .unwrap();
        self.client
            .log_message(MessageType::ERROR, format!("{params:?}"))
            .await;
    }
}

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    rt.block_on(main_async())
}

async fn main_async() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    let mut file = OpenOptions::new()
        .append(true)
        .open("/home/tim/Documents/temp/log.txt")
        .unwrap();
    file.write_all(format!("pls work before").into_bytes().as_slice())
        .unwrap();
    Server::new(stdin, stdout, socket).serve(service).await;
    let mut file = OpenOptions::new()
        .append(true)
        .open("/home/tim/Documents/temp/log.txt")
        .unwrap();
    file.write_all(format!("pls work after").into_bytes().as_slice())
        .unwrap();
}
