// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardStructureVersion } from "./CardStructureVersion";
import type { EquipmentTypeGen2 } from "./EquipmentTypeGen2";

/**
 * [DriverCardApplicationIdentification: appendix 2.61.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19751)
 */
export type DriverCardApplicationIdentificationGen2 = { typeOfTachographCardId: EquipmentTypeGen2, cardStructureVersion: CardStructureVersion, noOfEventsPerType: number, noOfFaultsPerType: number, activityStructureLength: number, noOfCardVehicleRecords: number, noOfCardPlaceRecords: number, noOfGnssAdRecords: number, noOfSpecificConditionRecords: number, noOfCardVehicleUnitRecords: number, };