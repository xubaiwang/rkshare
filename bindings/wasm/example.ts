import { sse_summary } from "./lib/rkshare_wasm.js";

const df = await sse_summary();

console.log(df);
console.log(df.getArrayMemorySize());
const ffi = df.intoFFI();

console.log(ffi);
