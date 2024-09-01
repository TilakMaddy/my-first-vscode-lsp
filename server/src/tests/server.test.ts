import { describe, afterEach, beforeEach, test } from "@jest/globals";
import { cwd } from "process";

import type {
  CodeAction,
  CompletionList,
  FullDocumentDiagnosticReport,
  Hover,
} from "vscode-languageserver";

import { LanguageServerWrapper } from "./language-server-wrapper";

let languageServer: LanguageServerWrapper; const init = async () => {
  await languageServer.request("initialize", {
    rootUri: "file:///home/user/project",
    capabilities: {}
  });
};

const documentVersion = new Map<string, number>();
const defaultFile = "file:///home/user/project/file.sol";
const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));


const didSave = (uri: string = defaultFile) => {
  languageServer.notify("textDocument/didSave", {
    textDocument: { uri }
  });
}


describe("lsp", () => {
  beforeEach(() => {
    languageServer = new LanguageServerWrapper(
      "cargo",
      ["run", "--quiet", "--manifest-path", `${cwd()}/../lsp_server/Cargo.toml`],
      true,
    );
    languageServer.start();
  });


  afterEach(() => {
    languageServer.stop();
  });


  test("initialization-works", async () => {
    await init();
  });


  test("didSave notification to lsp works", async () => {
    await init();
    didSave();
    const diagnostics = await languageServer.publishedDiagnostics();
    console.log(diagnostics);
  });

});
