// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ControlType } from "./ControlType";
import type { FullCardNumber } from "./FullCardNumber";
import type { TimeReal } from "./TimeReal";
import type { VehicleRegistrationIdentification } from "./VehicleRegistrationIdentification";

/**
 * [CardControlActivityDataRecord appendix 2.15.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17002)
 */
export type CardControlActivityDataRecord = { controlType: ControlType, controlTime: TimeReal | null, controlCardNumber: FullCardNumber, controlVehicleRegistration: VehicleRegistrationIdentification, controlDownloadPeriodBegin: TimeReal | null, controlDownloadPeriodEnd: TimeReal | null, };
