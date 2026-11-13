import wieland from "./wieland.ts";

type Handler = (event: unknown, ctx: unknown) => Promise<unknown>;

const handlers: Handler[] = [];
const calls: string[][] = [];

const target = process.argv[2] ?? "";
const fakePi = {
  on(name: string, handler: Handler): void {
    if (name === "tool_result") {
      handlers.push(handler);
    }
  },
  async exec(command: string, args: string[]): Promise<{ stdout: string; stderr: string; code: number }> {
    calls.push([command, ...args]);
    const { spawnSync } = await import("node:child_process");
    const run = spawnSync(process.env["WIELAND_BIN"] ?? command, args, { encoding: "utf8" });
    return { stdout: run.stdout, stderr: run.stderr, code: run.status ?? 1 };
  },
};

wieland(fakePi as never);
const handler = handlers[0];
if (handler === undefined) {
  throw new Error("extension registered no tool_result handler");
}
const patch = await handler(
  { toolName: "edit", input: { path: target }, content: [{ type: "text", text: "edited" }] },
  { signal: new AbortController().signal },
);
console.log(JSON.stringify({ calls, patch }));
