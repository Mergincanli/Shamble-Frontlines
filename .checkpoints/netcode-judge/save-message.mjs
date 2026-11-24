// Saves one of Mustafa's chat messages word for word, by script (never retyped), from the session's own record.
//   node .checkpoints/netcode-judge/save-message.mjs <session record .jsonl> <first words of the message> <out file>
// The record holds a message he sends while a turn runs as a line {"type":"queue-operation","content":"..."}; one he
// sends between turns as {"type":"user","message":{"content":...}}. The first line whose text starts with the given
// words is written, unchanged, under a three-line head. It refuses to run if no line or more than one text matches.
import { readFileSync, writeFileSync } from "node:fs";

const [record, start, out] = process.argv.slice(2);
if (!record || !start || !out) {
  console.error("usage: save-message.mjs <session record .jsonl> <first words of the message> <out file>");
  process.exit(2);
}
const texts = new Set();
for (const line of readFileSync(record, "utf8").split("\n")) {
  if (!line.includes(start.slice(0, 20))) continue;
  let row;
  try {
    row = JSON.parse(line);
  } catch {
    continue;
  }
  const content = row.type === "queue-operation" ? row.content : row.type === "user" ? row.message?.content : null;
  const parts = typeof content === "string" ? [content] : Array.isArray(content) ? content.map((c) => c?.text) : [];
  for (const text of parts) if (typeof text === "string" && text.startsWith(start)) texts.add(text);
}
if (texts.size !== 1) {
  console.error(`save-message: ${texts.size} different messages start with those words, not 1`);
  process.exit(1);
}
const [text] = texts;
const head = [
  "# Mustafa's message to Netcode Builder 2, 2026-11-24, word for word",
  "Saved by `.checkpoints/netcode-judge/save-message.mjs` from the session's record, as his item 3 asks. For whoever",
  "merges this branch: his decisions are kept in `docs/DECISIONS.md`, outside this chat's fence.",
  "",
];
writeFileSync(out, `${head.join("\n")}\n${text.trimEnd()}\n`);
console.log(`saved: ${out} (${text.length} characters)`);
