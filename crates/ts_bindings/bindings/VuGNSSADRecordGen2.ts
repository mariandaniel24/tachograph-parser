// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { GNSSPlaceRecordGen2 } from "./GNSSPlaceRecordGen2";
import type { OdometerShort } from "./OdometerShort";
import type { TimeReal } from "./TimeReal";

/**
 * [VuGNSSADRecord: appendix 2.203.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27345)
 */
export type VuGNSSADRecordGen2 = { timeStamp: TimeReal, cardNumberAndGenDriverSlot: FullCardNumberAndGenerationGen2 | null, cardNumberAndGenCodriverSlot: FullCardNumberAndGenerationGen2 | null, gnssPlaceRecord: GNSSPlaceRecordGen2, vehicleOdometerValue: OdometerShort, };
