use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ManufacturerCode: appendix 2.94.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22253)
pub struct ManufacturerCode(String);
impl ManufacturerCode {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let code = cursor
            .read_u8()
            .context("Failed to read ManufacturerCode")?;

        let name = match code {
            0x10 => "Actia S.A.",
            0x11 => "Security Printing and Systems Ltd.",
            0x12 => "Austria Card Plastikkarten und Ausweissysteme GmbH",
            0x13 => "Agencija za komercijalnu djelatnost d.o.o (AKD)",
            0x14 => "ALIK Automotive GmbH",
            0x15 => "ASELSAN",
            0x16 => "Asia Tacho Kart LLC",
            0x17 => "Real Casa de la Moneda",
            0x18 => "BARBÉ S.R.L.",
            0x19 => "BogArt Sp. z o.o.",
            0x20 => "CETIS d.d.",
            0x21 => "certSIGN",
            0x22 => "RUE Cryptotech",
            0x23 => "Centr Modernizatcii Transporta OOO (CMT - LLC)",
            0x24 => "Pars Ar-Ge Ltd",
            0x25 => "Cardplus Sverige AB",
            0x28 => "Datakom",
            0x29 => "DVLA",
            0x30 => "IDEMIA The Netherlands BV",
            0x32 => "EFKON AG",
            0x35 => "Eurocard LLC",
            0x38 => "Fábrica Nacional de Moneda y Timbre",
            0x39 => "First Print Yard",
            0x40 => "Giesecke & Devrient GmbH",
            0x43 => "Giesecke & Devrient GB Ltd.",
            0x44 => "Giesecke & Devrient sa/nv",
            0x45 => "GrafoCARD",
            0x48 => "Hungarian Banknote Printing Co. Ltd.",
            0x49 => "Haug GmbH",
            0x4A => "Hegard Sp. z o.o.",
            0x50 => "Imprimerie Nationale",
            0x51 => "Imprensa Nacional-Casa da Moeda, SA",
            0x52 => "InfoCamere S.C.p.A",
            0x53 => "Intellic Germany GmbH - ZF Group CVS",
            0x55 => "INTELLIGENT TELEMATICS SYSTEMS FOR TRANSPORT (its-t.llc)",
            0x60 => "Kraftfahrt-Bundesamt (KBA)",
            0x61 => "KazTACHOnet LLP",
            0x68 => "LESIKAR a.s.",
            0x69 => "LEDA-SL",
            0x78 => "NAP automotive Produkte GmbH",
            0x79 => "NATIONAL BANK OF SERBIA",
            0x81 => "Morpho e-documents",
            0x82 => "ORGA Zelenograd ZAO",
            0x84 => "ORGA Kartensysteme GmbH",
            0x88 => "Asseco - Central Europe a.s.",
            0x89 => "Polska Wytwórnia Papierów Wartosciowych S.A. - PWPW S.A.",
            0x8A => "Papiery Powlekane Pasaco Sp. z o.o.",
            0x98 => "TahoNetSoft",
            0xA1 => "Continental Automotive Technologies",
            0xA2 => "Stoneridge Electronics AB",
            0xA3 => "Thales",
            0xA4 => "3M Security Printing and Systems Ltd.",
            0xA5 => "STMicroelectronics - Incard Division",
            0xA6 => "STÁTNÍ TISKÁRNA CENIN, státní podnik",
            0xAB => "T-Systems International GmbH",
            0xAC => "Thales DIS Schweiz AG",
            0xAD => "Trüb Baltic AS",
            0xAE => "TEMPEST a.s.",
            0xAF => "Trueb - DEMAX PLC",
            0xB0 => "TAYROL LIMITED",
            0xB1 => "UŽDAROJI AKCINĖ BENDROVĖ \"LODVILA\"",
            0xD8 => "Union of Chambers and Commodity Exchanges of Turkey - TOBB",
            0xE0 => "Turker Roll Paper Trade",
            _ => anyhow::bail!("Unknown ManufacturerCode: {}", code),
        };
        Ok(ManufacturerCode(name.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [NationNumeric: appendix 2.101.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22450)
pub struct NationNumeric(String);
impl NationNumeric {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read nation numeric")?;
        // TODO: decide if we want to keep this list up to date, or just provide the raw value
        let parsed_country = match value {
            0x00 => "No information available",
            0x01 => "Austria",
            0x02 => "Albania",
            0x03 => "Andorra",
            0x04 => "Armenia",
            0x05 => "Azerbaijan",
            0x06 => "Belgium",
            0x07 => "Bulgaria",
            0x08 => "Bosnia Herzegovina",
            0x09 => "Belarus",
            0x0A => "Switzerland",
            0x0B => "Cyprus",
            0x0C => "Czech Republic",
            0x0D => "Germany",
            0x0E => "Denmark",
            0x0F => "Spain",
            0x10 => "Estonia",
            0x11 => "France",
            0x12 => "Finland",
            0x13 => "Liechtenstein",
            0x14 => "Faroe Islands",
            0x15 => "United Kingdom",
            0x16 => "Georgia",
            0x17 => "Greece",
            0x18 => "Hungary",
            0x19 => "Croatia",
            0x1A => "Italy",
            0x1B => "Ireland",
            0x1C => "Iceland",
            0x1D => "Kazakhstan",
            0x1E => "Luxembourg",
            0x1F => "Lithuania",
            0x20 => "Latvia",
            0x21 => "Malta",
            0x22 => "Monaco",
            0x23 => "Moldova",
            0x24 => "North Macedonia",
            0x25 => "Norway",
            0x26 => "Netherlands",
            0x27 => "Portugal",
            0x28 => "Poland",
            0x29 => "Romania",
            0x2A => "San Marino",
            0x2B => "Russia",
            0x2C => "Sweden",
            0x2D => "Slovakia",
            0x2E => "Slovenia",
            0x2F => "Turkmenistan",
            0x30 => "Türkiye",
            0x31 => "Ukraine",
            0x32 => "Vatican City",
            0x34 => "Montenegro",
            0x35 => "Serbia",
            0x36 => "Uzbekistan",
            0x37 => "Tajikistan",
            0x38 => "Kyrgyz Republic",
            0xFD => "European Community",
            0xFE => "Rest of Europe",
            0xFF => "Rest of the World",
            _ => "Reserved for Future Use",
        };
        Ok(NationNumeric(parsed_country.to_string()))
    }
}
