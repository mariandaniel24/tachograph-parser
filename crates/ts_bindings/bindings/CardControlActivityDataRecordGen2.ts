// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ControlTypeGen2 } from "./ControlTypeGen2";
import type { FullCardNumberGen2 } from "./FullCardNumberGen2";
import type { TimeReal } from "./TimeReal";
import type { VehicleRegistrationIdentification } from "./VehicleRegistrationIdentification";

/**
 * [CardControlActivityDataRecord appendix 2.15.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17002)
 */
export type CardControlActivityDataRecordGen2 = { controlType: ControlTypeGen2, controlTime: TimeReal | null, controlCardNumber: FullCardNumberGen2 | null, controlVehicleRegistration: VehicleRegistrationIdentification | null, controlDownloadPeriodBegin: TimeReal | null, controlDownloadPeriodEnd: TimeReal | null, };
