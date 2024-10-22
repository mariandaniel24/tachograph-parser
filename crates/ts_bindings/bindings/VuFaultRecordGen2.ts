// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { EventFaultRecordPurpose } from "./EventFaultRecordPurpose";
import type { EventFaultTypeGen2 } from "./EventFaultTypeGen2";
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { ManufacturerSpecificEventFaultDataGen2 } from "./ManufacturerSpecificEventFaultDataGen2";
import type { TimeReal } from "./TimeReal";

export type VuFaultRecordGen2 = { faultType: EventFaultTypeGen2, faultRecordPurpose: EventFaultRecordPurpose, faultBeginTime: TimeReal, faultEndTime: TimeReal, cardNumberAndGenDriverSlotBegin: FullCardNumberAndGenerationGen2 | null, cardNumberAndGenCodriverSlotBegin: FullCardNumberAndGenerationGen2 | null, cardNumberAndGenDriverSlotEnd: FullCardNumberAndGenerationGen2 | null, cardNumberAndGenCodriverSlotEnd: FullCardNumberAndGenerationGen2 | null, manufacturerSpecificEventFaultData: ManufacturerSpecificEventFaultDataGen2 | null, };
