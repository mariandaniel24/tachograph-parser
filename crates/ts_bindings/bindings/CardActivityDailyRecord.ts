// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardActivityChangeInfo } from "./CardActivityChangeInfo";
import type { DailyPresenceCounter } from "./DailyPresenceCounter";
import type { Distance } from "./Distance";
import type { TimeReal } from "./TimeReal";

/**
 * [CardDriverActivity: appendix 2.9.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16718)
 */
export type CardActivityDailyRecord = { activityPreviousRecordLength: number, activityRecordLength: number, activityRecordDate: TimeReal, activityDailyPresenceCounter: DailyPresenceCounter, activityDayDistance: Distance, activityChangeInfo: Array<CardActivityChangeInfo>, };
