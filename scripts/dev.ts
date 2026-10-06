const processes = [
  Bun.spawn(["bun", "run", "css:watch"], {
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }),
  Bun.spawn(["dx", "serve"], {
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  }),
];

let stopping = false;

async function stop(signal?: number) {
  if (stopping) return;
  stopping = true;

  for (const child of processes) {
    if (child.exitCode === null) child.kill();
  }

  await Promise.all(processes.map((child) => child.exited));
  if (signal !== undefined) process.exitCode = signal;
}

process.on("SIGINT", () => void stop(130));
process.on("SIGTERM", () => void stop(143));

const exitCode = await Promise.race(processes.map((child) => child.exited));
await stop(exitCode);