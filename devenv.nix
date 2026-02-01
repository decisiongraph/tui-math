{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/languages/
  languages.rust.enable = true;

  claude.code.enable = true;

  # Browser automation for testing
  claude.code.mcpServers.playwright = {
    type = "stdio";
    command = "npx";
    args = [ "@playwright/mcp@latest" ];
  };

  # Gemini UX review via consult-llm-mcp (requires GEMINI_API_KEY env var)
  # System prompt configured in ~/.consult-llm-mcp/SYSTEM_PROMPT.md
  claude.code.mcpServers.consult-llm = {
    type = "stdio";
    command = "npx";
    args = [ "-y" "consult-llm-mcp" ];
    env = {
      CONSULT_LLM_DEFAULT_MODEL = "gemini-3-pro-preview";
      CONSULT_LLM_ALLOWED_MODELS = "gemini-3-pro-preview,gemini-2.5-pro";
    };
  };
}
