import { expect, describe, it } from "bun:test";
import {
	parseTachoFile,
	parseVu,
	parseCard,
	detectTachoFileType,
	type TachoFileType,
	type TachoData,
	type VuData,
	type CardData,
} from "../index.js";
import type { Buffer } from "node:buffer";

describe("Tacho file parsing functions", () => {
	it("should have correct types for parseTachoFile", () => {
		type ParseTachoFileType = (bytes: Buffer) => TachoData;
		const _parseTachoFile: ParseTachoFileType = parseTachoFile;
	});

	it("should have correct types for parseVu", () => {
		type ParseVuType = (bytes: Buffer) => VuData;
		const _parseVu: ParseVuType = parseVu;
	});

	it("should have correct types for parseCard", () => {
		type ParseCardType = (path: string) => CardData;
		const _parseCard: ParseCardType = parseCard;
	});

	it("should have correct types for detectTachoFileType", () => {
		type DetectTachoFileTypeType = (bytes: Buffer) => TachoFileType;
		const _detectTachoFileType: DetectTachoFileTypeType = detectTachoFileType;
	});
});
