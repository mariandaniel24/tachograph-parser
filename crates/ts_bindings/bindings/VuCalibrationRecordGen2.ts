// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Address } from "./Address";
import type { CalibrationPurposeGen2 } from "./CalibrationPurposeGen2";
import type { FullCardNumberGen2 } from "./FullCardNumberGen2";
import type { KConstantOfRecordingEquipment } from "./KConstantOfRecordingEquipment";
import type { LTyreCircumference } from "./LTyreCircumference";
import type { Name } from "./Name";
import type { OdometerShort } from "./OdometerShort";
import type { SealDataVuGen2 } from "./SealDataVuGen2";
import type { Speed } from "./Speed";
import type { TimeReal } from "./TimeReal";
import type { TyreSize } from "./TyreSize";
import type { VehicleIdentificationNumber } from "./VehicleIdentificationNumber";
import type { VehicleRegistrationIdentification } from "./VehicleRegistrationIdentification";
import type { WVehicleCharacteristicConstant } from "./WVehicleCharacteristicConstant";

export type VuCalibrationRecordGen2 = { calibrationPurpose: CalibrationPurposeGen2, workshopName: Name, workshopAddress: Address, workshopCardNumber: FullCardNumberGen2, workshopCardExpiryDate: TimeReal | null, vehicleIdentificationNumber: VehicleIdentificationNumber | null, vehicleRegistrationIdentification: VehicleRegistrationIdentification | null, wVehicleCharacteristicConstant: WVehicleCharacteristicConstant, kConstantOfRecordingEquipment: KConstantOfRecordingEquipment, lTyreCircumference: LTyreCircumference, tyreSize: TyreSize, authorisedSpeed: Speed, oldOdometerValue: OdometerShort, newOdometerValue: OdometerShort, oldTimeValue: TimeReal | null, newTimeValue: TimeReal | null, nextCalibrationDate: TimeReal | null, sealDataVu: SealDataVuGen2, };
