import { tableFromJSON } from "npm:apache-arrow";

const table = tableFromJSON([
  { foo: 123 },
  { bar: 233 },
]);

console.table([...table]);
console.log(table.getChild("foo")?.toArray());
