use log::{info, LevelFilter};
use simple_logging::log_to_file;
use tower_lsp::jsonrpc::Result;
// use tower_lsp::lsp_types::notification::PublishDiagnostics;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        info!("didSave caught : {:?}", params);

        let diagnostic = Diagnostic::new_simple(
            Range {
                start: Position {
                    line: 1,
                    character: 3,
                },
                end: Position {
                    line: 2,
                    character: 20,
                },
            },
            "BAD CODE".to_string(),
        );

        self.client
            .publish_diagnostics(
                Url::parse("file:///URI-FROM-RUST").unwrap(),
                vec![diagnostic],
                None,
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    _ = log_to_file("lsp_server.log", LevelFilter::Info);

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
