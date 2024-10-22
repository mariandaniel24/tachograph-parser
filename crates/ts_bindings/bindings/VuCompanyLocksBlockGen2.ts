// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SensorExternalGNSSCoupledRecordGen2 } from "./SensorExternalGNSSCoupledRecordGen2";
import type { SensorPairedRecordGen2 } from "./SensorPairedRecordGen2";
import type { SignatureGen2 } from "./SignatureGen2";
import type { VuCalibrationRecordGen2 } from "./VuCalibrationRecordGen2";
import type { VuCardRecordGen2 } from "./VuCardRecordGen2";
import type { VuITSConsentRecordGen2 } from "./VuITSConsentRecordGen2";
import type { VuIdentificationGen2 } from "./VuIdentificationGen2";
import type { VuPowerSupplyInterruptionRecordGen2 } from "./VuPowerSupplyInterruptionRecordGen2";

export type VuCompanyLocksBlockGen2 = { vuIdentificationRecordArray: Array<VuIdentificationGen2>, vuSensorPairedRecordArray: Array<SensorPairedRecordGen2>, vuSensorExternalGnssCoupledRecordArray: Array<SensorExternalGNSSCoupledRecordGen2>, vuCalibrationRecordArray: Array<VuCalibrationRecordGen2>, vuCardRecordArray: Array<VuCardRecordGen2>, vuItsConsentRecordArray: Array<VuITSConsentRecordGen2>, vuPowerSupplyInterruptionRecordArray: Array<VuPowerSupplyInterruptionRecordGen2>, signatureRecordArray: Array<SignatureGen2>, };
