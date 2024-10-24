// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardChipIdentification } from "./CardChipIdentification";
import type { CardControlActivityDataRecord } from "./CardControlActivityDataRecord";
import type { CardDownload } from "./CardDownload";
import type { CardDrivingLicenceInformation } from "./CardDrivingLicenceInformation";
import type { CardEventData } from "./CardEventData";
import type { CardFaultData } from "./CardFaultData";
import type { CardIccIdentification } from "./CardIccIdentification";
import type { CardPlaceDailyWorkPeriod } from "./CardPlaceDailyWorkPeriod";
import type { CardVehiclesUsed } from "./CardVehiclesUsed";
import type { Certificate } from "./Certificate";
import type { CurrentUsage } from "./CurrentUsage";
import type { DriverActivityData } from "./DriverActivityData";
import type { DriverCardApplicationIdentification } from "./DriverCardApplicationIdentification";
import type { Identification } from "./Identification";
import type { Signature } from "./Signature";
import type { SpecificConditions } from "./SpecificConditions";

export type CardGen1Blocks = { cardIccIdentification: CardIccIdentification, cardChipIdentification: CardChipIdentification, applicationIdentification: DriverCardApplicationIdentification, applicationIdentificationSignature: Signature, cardCertificate: Certificate, memberStateCertificate: Certificate, identification: Identification, identificationSignature: Signature, cardDownload: CardDownload | null, cardDownloadSignature: Signature | null, driverLicenceInfo: CardDrivingLicenceInformation | null, driverLicenceInfoSignature: Signature | null, eventsData: CardEventData, eventsDataSignature: Signature, faultsData: CardFaultData, faultsDataSignature: Signature, driverActivityData: DriverActivityData, driverActivityDataSignature: Signature, vehiclesUsed: CardVehiclesUsed, vehiclesUsedSignature: Signature, places: CardPlaceDailyWorkPeriod, placesSignature: Signature, currentUsage: CurrentUsage | null, currentUsageSignature: Signature | null, controlActivityData: CardControlActivityDataRecord, controlActivityDataSignature: Signature, specificConditions: SpecificConditions, specificConditionsSignature: Signature, };
