// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { TimeReal } from "./TimeReal";
import type { VuSoftwareVersion } from "./VuSoftwareVersion";

/**
 * [VuSoftwareIdentification: appendix 2.225.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28538)
 */
export type VuSoftwareIdentification = { vuSoftwareVersion: VuSoftwareVersion, vuSoftInstallationDate: TimeReal, };