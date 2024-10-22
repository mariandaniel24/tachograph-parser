// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardNumber } from "./CardNumber";
import type { Name } from "./Name";
import type { NationNumeric } from "./NationNumeric";
import type { TimeReal } from "./TimeReal";

/**
 * [CardIdentification: appendix 2.24.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17430)
 */
export type CardIdentification = { cardIssuingMemberState: NationNumeric, cardNumber: CardNumber, cardIssuingAuthorityName: Name, cardIssueDate: TimeReal, cardValidityBegin: TimeReal, cardExpiryDate: TimeReal, };
