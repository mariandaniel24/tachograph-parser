// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SensorPaired } from "./SensorPaired";
import type { Signature } from "./Signature";
import type { VuCalibrationData } from "./VuCalibrationData";
import type { VuIdentification } from "./VuIdentification";

/**
 * [VuCompanyLocksBlock: appendix 2.236.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28868)
 */
export type VuCompanyLocksBlock = { vuIdentification: VuIdentification, sensorPaired: SensorPaired, vuCalibrationData: VuCalibrationData, signature: Signature, };
