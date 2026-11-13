import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

const EDIT_TOOLS = new Set(["edit", "write"]);

function editedPath(input: unknown): string | undefined {
  if (typeof input !== "object" || input === null || !("path" in input)) {
    return undefined;
  }
  return typeof input.path === "string" ? input.path : undefined;
}

export default function wieland(pi: ExtensionAPI): void {
  pi.on("tool_result", async (event, ctx) => {
    if (!EDIT_TOOLS.has(event.toolName)) {
      return undefined;
    }
    const path = editedPath(event.input);
    if (path === undefined) {
      return undefined;
    }
    const result = await pi.exec("wieland", ["hook", "--agent", "pi", "--path", path], {
      signal: ctx.signal,
      timeout: 120_000,
    });
    if (result.stdout.trim() === "") {
      return undefined;
    }
    return {
      content: [...event.content, { type: "text", text: result.stdout }],
      isError: result.code === 2,
    };
  });
}
