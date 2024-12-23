// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardSlotsStatus } from "./CardSlotsStatus";
import type { Certificate } from "./Certificate";
import type { Signature } from "./Signature";
import type { TimeReal } from "./TimeReal";
import type { VehicleIdentificationNumber } from "./VehicleIdentificationNumber";
import type { VehicleRegistrationIdentification } from "./VehicleRegistrationIdentification";
import type { VuCompanyLocksData } from "./VuCompanyLocksData";
import type { VuControlActivityData } from "./VuControlActivityData";
import type { VuDownloadActivityData } from "./VuDownloadActivityData";
import type { VuDownloadablePeriod } from "./VuDownloadablePeriod";

/**
 * [VuOverviewBlock page 342]
 */
export type VuOverviewBlock = { memberStateCertificate: Certificate, vuCertificate: Certificate, vehicleIdentificationNumber: VehicleIdentificationNumber, vehicleRegistrationIdentification: VehicleRegistrationIdentification, currentDateTime: TimeReal, vuDownloadablePeriod: VuDownloadablePeriod, cardSlotsStatus: CardSlotsStatus, vuDownloadActivityData: VuDownloadActivityData, vuCompanyLocksData: VuCompanyLocksData, vuControlActivityData: VuControlActivityData, signature: Signature, };
