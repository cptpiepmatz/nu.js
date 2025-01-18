import fs from "node:fs";

const JS_FILE_PATH = "pkg/deno/nu_js.js";
const PROVIDER = `// @ts-self-types="./nu_js.d.ts"`

const content = fs.readFileSync(JS_FILE_PATH, "utf-8");
const newContent = `${PROVIDER}\n${content}`;

fs.writeFileSync(JS_FILE_PATH, newContent);
