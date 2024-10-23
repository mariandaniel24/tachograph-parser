// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Address } from "./Address";
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { Name } from "./Name";

/**
 * [VuTimeAdjustmentRecord: appendix 2.232.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28728) 
 */
export type VuTimeAdjustmentRecordGen2 = { oldTimeValue: string, newTimeValue: string, workshopName: Name, workshopAddress: Address, workshopCardNumberAndGeneration: FullCardNumberAndGenerationGen2 | null, };