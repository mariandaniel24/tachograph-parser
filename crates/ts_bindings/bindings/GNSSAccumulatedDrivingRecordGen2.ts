// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { GNSSPlaceRecordGen2 } from "./GNSSPlaceRecordGen2";
import type { OdometerShort } from "./OdometerShort";
import type { TimeReal } from "./TimeReal";

/**
 * [GNSSAccumulatedDrivingRecord: appendix 2.79.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21640)
 */
export type GNSSAccumulatedDrivingRecordGen2 = { timeStamp: TimeReal, gnssPlaceRecord: GNSSPlaceRecordGen2, vehicleOdometerValue: OdometerShort, };
