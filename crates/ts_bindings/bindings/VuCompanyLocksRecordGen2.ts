// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Address } from "./Address";
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { Name } from "./Name";
import type { TimeReal } from "./TimeReal";

/**
 * [VuCompanyLocksRecord: appendix 2.184.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26187)
 */
export type VuCompanyLocksRecordGen2 = { lockInTime: TimeReal, lockOutTime: TimeReal | null, companyName: Name, companyAddress: Address, companyCardNumberAndGeneration: FullCardNumberAndGenerationGen2, };