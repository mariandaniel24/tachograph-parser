import Bun from "bun";

const indexDtsPath = "./index.d.ts";
const indexJsPath = "./index.js";

// Read the contents of index.d.ts
const content = await Bun.file(indexDtsPath).text();

// Define the import statements to be added
const importStatements = `import type { VuData } from "./bindings/VuData";
import type { CardData } from "./bindings/CardData";
import type { TachoFileType } from "./bindings/TachoFileType";
`;

// Combine the import statements with the existing content
const updatedContent = importStatements + content;

// Write the updated content back to index.d.ts
await Bun.write(indexDtsPath, updatedContent);

console.log("index.d.ts has been updated with the import statements.");

// Read the contents of index.js
let jsContent = await Bun.file(indexJsPath).text();

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
await Bun.write(indexJsPath, jsContent);

console.log("index.js has been updated with JSON.parse wrapping.");
