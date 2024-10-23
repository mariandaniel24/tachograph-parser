import { it } from "node:test";
import { strict as assert } from "node:assert";
import { parseVu, parseCard, detectTachoFileType } from "../index.js";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";

it("should parse a tacho file with parseCard in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(readFileSync("../../data/card_gen2.ddd"));
	const start = performance.now();
	const parsed = parseCard(tachoFileBuffer);
	const end = performance.now();
	const duration = end - start;
	console.log(
		`Parsing with parseCard took ${duration.toFixed(2)} milliseconds`,
	);

	assert(duration < 100_000_000);
});
it("should parse a tacho file with parseVu in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(readFileSync("../../data/vu_gen2.ddd"));
	const start = performance.now();
	const parsed = parseVu(tachoFileBuffer);
	const end = performance.now();
	const duration = end - start;
	console.log(`Parsing with parseVu took ${duration.toFixed(2)} milliseconds`);

	assert(duration < 100_000_000);
});
it("should detect a tacho file type in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(readFileSync("../../data/card_gen2.ddd"));
	const start = performance.now();
	const parsed = detectTachoFileType(tachoFileBuffer);
	const end = performance.now();

	const duration = end - start;
	console.log(
		`Detecting tacho file type took ${duration.toFixed(2)} milliseconds`,
	);
	assert(duration < 100_000_000);
});
