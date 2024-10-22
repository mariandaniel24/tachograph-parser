import { it, expect } from "bun:test";
import { parseVu, parseCard, detectTachoFileType } from "../index.js";
import { Buffer } from "node:buffer";

it("should parse a tacho file with parseCard in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(
		await Bun.file("../../data/card_gen2.ddd").arrayBuffer(),
	);
	const start = performance.now();
	const parsed = parseCard(tachoFileBuffer);
	const end = performance.now();
	const duration = end - start;
	console.log(
		`Parsing with parseCard took ${duration.toFixed(2)} milliseconds`,
	);

	expect(duration).toBeLessThan(100_000_000);
});
it("should parse a tacho file with parseVu in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(
		await Bun.file("../../data/vu_gen2.ddd").arrayBuffer(),
	);
	const start = performance.now();
	const parsed = parseVu(tachoFileBuffer);
	const end = performance.now();
	const duration = end - start;
	console.log(`Parsing with parseVu took ${duration.toFixed(2)} milliseconds`);

	expect(duration).toBeLessThan(100_000_000);
});
it("should detect a tacho file type in under 100ms", async () => {
	const tachoFileBuffer = Buffer.from(
		await Bun.file("../../data/card_gen2.ddd").arrayBuffer(),
	);
	const start = performance.now();
	const parsed = detectTachoFileType(tachoFileBuffer);
	const end = performance.now();

	const duration = end - start;
	console.log(
		`Detecting tacho file type took ${duration.toFixed(2)} milliseconds`,
	);
	expect(duration).toBeLessThan(100_000_000);
});
