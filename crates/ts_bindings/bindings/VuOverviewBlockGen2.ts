// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardSlotsStatus } from "./CardSlotsStatus";
import type { CertificateGen2 } from "./CertificateGen2";
import type { CurrentDateTime } from "./CurrentDateTime";
import type { SignatureGen2 } from "./SignatureGen2";
import type { VehicleIdentificationNumber } from "./VehicleIdentificationNumber";
import type { VehicleRegistrationNumber } from "./VehicleRegistrationNumber";
import type { VuCompanyLocksRecordGen2 } from "./VuCompanyLocksRecordGen2";
import type { VuControlActivityRecordGen2 } from "./VuControlActivityRecordGen2";
import type { VuDownloadActivityDataGen2 } from "./VuDownloadActivityDataGen2";
import type { VuDownloadablePeriod } from "./VuDownloadablePeriod";

/**
 * Generation 2, version 1 (TREP 0x21); page 342
 */
export type VuOverviewBlockGen2 = { 
/**
 * Member state certificate
 */
memberStateCertificateRecordArray: Array<CertificateGen2>, 
/**
 * VU certificate
 */
vuCertificateRecordArray: Array<CertificateGen2>, 
/**
 * Vehicle identification
 */
vehicleIdentificationNumberRecordArray: Array<VehicleIdentificationNumber>, 
/**
 * Vehicle registration number
 */
vehicleRegistrationNumberRecordArray: Array<VehicleRegistrationNumber>, 
/**
 * VU current date and time
 */
currentDateTimeRecordArray: Array<CurrentDateTime>, 
/**
 * Downloadable period
 */
vuDownloadablePeriodRecordArray: Array<VuDownloadablePeriod>, 
/**
 * Type of cards inserted in the VU
 */
cardSlotsStatusRecordArray: Array<CardSlotsStatus>, 
/**
 * Previous VU download
 */
vuDownloadActivityDataRecordArray: Array<VuDownloadActivityDataGen2>, 
/**
 * All company locks stored.
 */
vuCompanyLocksRecordArray: Array<VuCompanyLocksRecordGen2>, 
/**
 * All control records stored in the VU.
 */
vuControlActivityRecordArray: Array<VuControlActivityRecordGen2>, 
/**
 * ECC signature of all preceding data except the certificates
 */
signatureRecordArray: Array<SignatureGen2>, };
