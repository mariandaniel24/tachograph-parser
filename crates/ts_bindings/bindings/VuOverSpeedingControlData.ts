// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Speed } from "./Speed";
import type { TimeReal } from "./TimeReal";

/**
 * [VuOverSpeedingControlData: appendix 2.212.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27978)
 */
export type VuOverSpeedingControlData = { lastOverspeedControlTime: TimeReal | null, firstOverspeedSince: TimeReal | null, numberOfOverspeedSince: Speed, };
