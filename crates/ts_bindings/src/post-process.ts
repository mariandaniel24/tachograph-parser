import * as fs from "node:fs/promises";
import * as path from "node:path";

// List of files to process
const filesToProcess = [
	"../parser/src/dt/gen1.rs",
	"../parser/src/dt/gen2.rs",
	"../parser/src/dt/gen2v2.rs",
	"../parser/src/dt/mod.rs",
];
// Keep track of the types to add
const typesToAdd = new Set<string>();

// Go through each file
for (const file of filesToProcess) {
	// Read the file
	const content = await Bun.file(file).text();
	const lines = content.split("\n");

	// Go line by line
	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];
		// Find all type aliases
		if (line.trim().startsWith("pub type")) {
			// Capture all the doc comments above the type alias
			const comments = [];
			for (let j = i - 1; j >= 0; j--) {
				const line = lines[j].trim();
				if (!line.startsWith("///")) break;
				comments.unshift(line.substring(3).trim());
			}

			// Match the type alias
			const match = line.match(/pub type (\w+) = (\w+);/);
			// if it's a match, extract the type name and type value
			if (match) {
				const [, typeName, typeValue] = match;
				// convert rust number types (u8, u16, u32, u64) to JS number, everything else should be kept as is
				const jsType = ["u8", "u16", "u32", "u64"].includes(typeValue)
					? "number"
					: typeValue;
				// add the type alias to the set with comments
				const typeDeclaration =
					comments.length > 0
						? `/** ${comments.join("\n * ")} */\nexport type ${typeName} = ${jsType};`
						: `export type ${typeName} = ${jsType};`;
				typesToAdd.add(typeDeclaration);
			}
		}
	}
}

// Read the index.d.ts file
const indexDtsPath = path.join(__dirname, "../index.d.ts");
let indexDtsContent = await fs.readFile(indexDtsPath, "utf-8");

// Add the new types to the index.d.ts file
const newTypes = Array.from(typesToAdd).join("\n\n");
indexDtsContent = `${indexDtsContent}\n\n// Post processed types from src/post-process.ts \n\n${newTypes}`;

// Write the updated index.d.ts file
await fs.writeFile(indexDtsPath, indexDtsContent);
