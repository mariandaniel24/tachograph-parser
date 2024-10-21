import { describe, it, expect } from "bun:test";
import { parseCard } from "../index.js";
import { Buffer } from "node:buffer";

describe("Tacho file parsing performance", () => {
	it("should parse a tacho file in under 100ms", async () => {
		const tachoFileBuffer = Buffer.from(
			await Bun.file("../../data/card_gen2.ddd").arrayBuffer(),
		);
		const perf = Bun.nanoseconds();
		const parsed = parseCard(tachoFileBuffer);
		const duration = Bun.nanoseconds() - perf;
		console.log(`Parsing took ${duration / 1_000_000} milliseconds`);
		expect(duration).toBeLessThan(100_000_000);
	});
});
