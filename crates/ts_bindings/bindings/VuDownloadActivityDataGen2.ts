// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { FullCardNumberAndGenerationGen2 } from "./FullCardNumberAndGenerationGen2";
import type { Name } from "./Name";
import type { TimeReal } from "./TimeReal";

/**
 * [VuDownloadActivityData: appendix 2.195.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26758)
 */
export type VuDownloadActivityDataGen2 = { downloadingTime: TimeReal | null, fullCardNumberAndGeneration: FullCardNumberAndGenerationGen2 | null, companyOrWorkshopName: Name | null, };
