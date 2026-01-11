use crate::ipc::IpcClient;
use crate::mcp::protocol::*;
use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct McpServer {
    ipc_client: IpcClient,
}

impl McpServer {
    pub async fn new() -> Result<Self> {
        let ipc_client = IpcClient::connect().await?;
        Ok(Self { ipc_client })
    }

    pub async fn run(&self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let response = match serde_json::from_str::<JsonRpcRequest>(line) {
                Ok(request) => self.handle_request(&request).await,
                Err(e) => JsonRpcResponse::error(None, -32700, format!("Parse error: {}", e)),
            };

            let response_json = serde_json::to_string(&response)?;
            stdout.write_all(response_json.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }

        Ok(())
    }

    async fn handle_request(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request),
            "initialized" => JsonRpcResponse::success(request.id.clone(), json!({})),
            "tools/list" => self.handle_list_tools(request),
            "tools/call" => self.handle_call_tool(request).await,
            "ping" => JsonRpcResponse::success(request.id.clone(), json!({})),
            _ => JsonRpcResponse::error(
                request.id.clone(),
                -32601,
                format!("Method not found: {}", request.method),
            ),
        }
    }

    fn handle_initialize(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let result = InitializeResult {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
            },
            server_info: ServerInfo {
                name: SERVER_NAME.to_string(),
                version: SERVER_VERSION.to_string(),
            },
            instructions: Some(
                "MCP server for ghview - a GitHub PR viewer application. \
                 Use the 'screenshot' tool to capture the ghview window."
                    .to_string(),
            ),
        };

        JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
    }

    fn handle_list_tools(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let tools = vec![Tool {
            name: "screenshot".to_string(),
            description:
                "Capture a screenshot of the ghview window and save it to the specified directory"
                    .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "output_dir": {
                        "type": "string",
                        "description": "Directory to save the screenshot file"
                    }
                },
                "required": ["output_dir"]
            }),
        }];

        let result = ListToolsResult { tools };
        JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
    }

    async fn handle_call_tool(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let params: CallToolParams = match request.params.as_ref() {
            Some(p) => match serde_json::from_value(p.clone()) {
                Ok(params) => params,
                Err(e) => {
                    return JsonRpcResponse::error(
                        request.id.clone(),
                        -32602,
                        format!("Invalid params: {}", e),
                    );
                }
            },
            None => {
                return JsonRpcResponse::error(request.id.clone(), -32602, "Missing params");
            }
        };

        let result = match params.name.as_str() {
            "screenshot" => self.call_screenshot(&params.arguments).await,
            _ => CallToolResult::error(format!("Unknown tool: {}", params.name)),
        };

        JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
    }

    async fn call_screenshot(&self, arguments: &Option<Value>) -> CallToolResult {
        let output_dir = match arguments {
            Some(args) => match args.get("output_dir").and_then(|v| v.as_str()) {
                Some(dir) => dir.to_string(),
                None => return CallToolResult::error("Missing required argument: output_dir"),
            },
            None => return CallToolResult::error("Missing arguments"),
        };

        match self.ipc_client.screenshot(&output_dir).await {
            Ok(result) => CallToolResult::success(result),
            Err(e) => CallToolResult::error(format!("Screenshot failed: {}", e)),
        }
    }
}
