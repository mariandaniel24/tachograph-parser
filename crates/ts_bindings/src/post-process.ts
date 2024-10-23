import fs from "node:fs";

const indexDtsPath = "./index.d.ts";
const indexJsPath = "./index.js";

// Read the contents of index.d.ts
const content = fs.readFileSync(indexDtsPath, "utf8");

// Define the import statements to be added
const importStatements = `import type { VuData } from "./bindings/VuData";
import type { CardData } from "./bindings/CardData";
import type { TachoFileType } from "./bindings/TachoFileType";
`;

// Combine the import statements with the existing content
const updatedContent = importStatements + content;

// Write the updated content back to index.d.ts
fs.writeFileSync(indexDtsPath, updatedContent);

console.log("index.d.ts has been updated with the import statements.");

// Read the contents of index.js
let jsContent = fs.readFileSync(indexJsPath, "utf8");

// Define the new export statements
jsContent = jsContent.replace(
	"module.exports.parseCard = nativeBinding.parseCard",
	"module.exports.parseCard = (...input) => JSON.parse(nativeBinding.parseCard(...input))",
);
jsContent = jsContent.replace(
	"module.exports.parseVu = nativeBinding.parseVu",
	"module.exports.parseVu = (...input) => JSON.parse(nativeBinding.parseVu(...input))",
);

// Write the updated content back to index.js
fs.writeFileSync(indexJsPath, jsContent);

console.log("index.js has been updated with JSON.parse wrapping.");
