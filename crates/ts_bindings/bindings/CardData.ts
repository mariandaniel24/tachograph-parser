// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CardGen1Blocks } from "./CardGen1Blocks";
import type { CardGen2Blocks } from "./CardGen2Blocks";
import type { CardGen2V2Blocks } from "./CardGen2V2Blocks";

export type CardData = { "generation": "gen1", gen1Blocks: CardGen1Blocks, } | { "generation": "gen2", gen1Blocks: CardGen1Blocks, gen2Blocks: CardGen2Blocks, } | { "generation": "gen2V2", gen1Blocks: CardGen1Blocks, gen2Blocks: CardGen2Blocks, gen2v2Blocks: CardGen2V2Blocks, };
