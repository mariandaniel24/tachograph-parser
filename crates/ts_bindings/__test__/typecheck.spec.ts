import { describe, it } from "node:test";
import { parseVu, parseCard, detectTachoFileType } from "../index.js";
import type { Buffer } from "node:buffer";
import type { VuData } from "../bindings/VuData.js";
import type { CardData } from "../bindings/CardData.js";
import type { TachoFileType } from "../bindings/TachoFileType.js";

it("should have correct types for parseVu", () => {
	type ParseVuType = (bytes: Buffer) => VuData;
	const _parseVu: ParseVuType = parseVu;
});

it("should have correct types for parseCard", () => {
	type ParseCardType = (bytes: Buffer) => CardData;
	const _parseCard: ParseCardType = parseCard;
});

it("should have correct types for detectTachoFileType", () => {
	type DetectTachoFileTypeType = (bytes: Buffer) => TachoFileType;
	const _detectTachoFileType: DetectTachoFileTypeType = detectTachoFileType;
});
