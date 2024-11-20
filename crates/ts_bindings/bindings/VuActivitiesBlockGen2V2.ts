// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardActivityChangeInfo } from "./CardActivityChangeInfo";
import type { DateOfDayDownloadedGen2 } from "./DateOfDayDownloadedGen2";
import type { OdometerShort } from "./OdometerShort";
import type { SignatureGen2 } from "./SignatureGen2";
import type { SpecificConditionRecordGen2 } from "./SpecificConditionRecordGen2";
import type { VuBorderCrossingRecord } from "./VuBorderCrossingRecord";
import type { VuCardIwRecordGen2 } from "./VuCardIwRecordGen2";
import type { VuGNSSADRecordGen2V2 } from "./VuGNSSADRecordGen2V2";
import type { VuLoadUnloadRecord } from "./VuLoadUnloadRecord";
import type { VuPlaceDailyWorkPeriodRecordGen2V2 } from "./VuPlaceDailyWorkPeriodRecordGen2V2";

export type VuActivitiesBlockGen2V2 = { 
/**
 * Date of day downloaded
 */
dateOfDayDownloadedRecordArray: Array<DateOfDayDownloadedGen2>, 
/**
 * Odometer at end of downloaded day
 */
odometerValueMidnightRecordArray: Array<OdometerShort>, 
/**
 * Cards insertion withdrawal cycles data. If no data available, array has noOfRecords = 0.
 * When a record crosses 00:00 (insertion on previous day) or 24:00 (withdrawal next day),
 * it appears in full within both days involved.
 */
vuCardIwRecordArray: Array<VuCardIwRecordGen2>, 
/**
 * Slots status at 00:00 and activity changes recorded for the day downloaded
 */
vuActivityDailyRecordArray: Array<CardActivityChangeInfo>, 
/**
 * Places related data recorded for the day downloaded.
 */
vuPlaceDailyWorkPeriodRecordArray: Array<VuPlaceDailyWorkPeriodRecordGen2V2>, 
/**
 * GNSS positions when accumulated driving time reaches multiple of 3 hours.
 */
vuGnssAdRecordArray: Array<VuGNSSADRecordGen2V2>, 
/**
 * Specific conditions data recorded for the day downloaded.
 */
vuSpecificConditionRecordArray: Array<SpecificConditionRecordGen2>, 
/**
 * Border crossings for the day downloaded.
 */
vuBorderCrossingRecordArray: Array<VuBorderCrossingRecord>, 
/**
 * Load/unload operations for the day downloaded.
 */
vuLoadUnloadRecordArray: Array<VuLoadUnloadRecord>, 
/**
 * ECC signature of all preceding data
 */
signatureRecordArray: Array<SignatureGen2>, };