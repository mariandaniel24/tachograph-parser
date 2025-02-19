// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CalibrationPurpose } from "./CalibrationPurpose";
import type { ExtendedSerialNumber } from "./ExtendedSerialNumber";
import type { KConstantOfRecordingEquipment } from "./KConstantOfRecordingEquipment";
import type { LTyreCircumference } from "./LTyreCircumference";
import type { OdometerShort } from "./OdometerShort";
import type { Speed } from "./Speed";
import type { TimeReal } from "./TimeReal";
import type { TyreSize } from "./TyreSize";
import type { VehicleIdentificationNumber } from "./VehicleIdentificationNumber";
import type { VehicleRegistrationIdentification } from "./VehicleRegistrationIdentification";
import type { VuPartNumber } from "./VuPartNumber";
import type { WVehicleCharacteristicConstant } from "./WVehicleCharacteristicConstant";

export type WorkshopCardCalibrationRecord = { calibrationPurpose: CalibrationPurpose, vehicleIdentificationNumber: VehicleIdentificationNumber, vehicleRegistration: VehicleRegistrationIdentification, wVehicleCharacteristicConstant: WVehicleCharacteristicConstant, kConstantOfRecordingEquipment: KConstantOfRecordingEquipment, lTyreCircumference: LTyreCircumference, tyreSize: TyreSize, authorisedSpeed: Speed, oldOdometerValue: OdometerShort, newOdometerValue: OdometerShort, oldTimeValue: TimeReal, newTimeValue: TimeReal, nextCalibrationDate: TimeReal, vuPartNumber: VuPartNumber, vuSerialNumber: ExtendedSerialNumber, sensorSerialNumber: ExtendedSerialNumber, };
