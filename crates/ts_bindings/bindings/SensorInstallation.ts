// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ExtendedSerialNumber } from "./ExtendedSerialNumber";
import type { TimeReal } from "./TimeReal";
import type { VuApprovalNumber } from "./VuApprovalNumber";

/**
 * [SensorInstallation: appendix 2.141.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24238)
 */
export type SensorInstallation = { sensorPairingDateFirst: TimeReal, firstVuApprovalNumber: VuApprovalNumber, firstVuSerialNumber: ExtendedSerialNumber, sensorPairingDateCurrent: TimeReal, currentVuApprovalNumber: VuApprovalNumber, currentVuSerialNumber: ExtendedSerialNumber, };