// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardNumber } from "./CardNumber";
import type { EquipmentType } from "./EquipmentType";
import type { NationNumeric } from "./NationNumeric";

/**
 * [FullCardNumber: appendix 2.73.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21400)
 */
export type FullCardNumber = { cardType: EquipmentType, cardIssuingMemberState: NationNumeric, cardNumber: CardNumber, };
