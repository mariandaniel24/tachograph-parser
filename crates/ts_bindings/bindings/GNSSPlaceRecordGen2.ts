// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { GeoCoordinatesGen2 } from "./GeoCoordinatesGen2";
import type { GnssAccuracyGen2 } from "./GnssAccuracyGen2";
import type { TimeReal } from "./TimeReal";

/**
 * [GNSSPlaceRecord: appendix 2.80.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21772)
 */
export type GNSSPlaceRecordGen2 = { timeStamp: TimeReal, gnssAccuracy: GnssAccuracyGen2, geoCoordinates: GeoCoordinatesGen2, };
