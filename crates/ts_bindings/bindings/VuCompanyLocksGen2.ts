// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Address } from "./Address";
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { Name } from "./Name";
import type { TimeReal } from "./TimeReal";

export type VuCompanyLocksGen2 = { lockInTime: TimeReal, lockOutTime: TimeReal | null, companyName: Name, companyAddress: Address, companyCardNumberAndGeneration: FullCardNumberAndGenerationGen2, };
