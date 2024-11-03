// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { GNSSPlaceAuthRecord } from "./GNSSPlaceAuthRecord";
import type { OdometerShort } from "./OdometerShort";
import type { OperationType } from "./OperationType";
import type { TimeReal } from "./TimeReal";

/**
 * [VuLoadUnloadRecord: appendix 2.208a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27834)
 */
export type VuLoadUnloadRecord = { timeStamp: TimeReal, operationType: OperationType, cardNumberAndGenDriverSlot: FullCardNumberAndGenerationGen2 | null, cardNumberAndGenCodriverSlot: FullCardNumberAndGenerationGen2 | null, gnssPlaceAuthRecord: GNSSPlaceAuthRecord, vehicleOdometerValue: OdometerShort, };
