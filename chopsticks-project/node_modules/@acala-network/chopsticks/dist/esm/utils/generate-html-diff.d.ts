import type { Block } from '@acala-network/chopsticks-core';
import type { HexString } from '@polkadot/util/types';
export declare const generateHtmlDiff: (block: Block, diff: [HexString, HexString | null][]) => Promise<string>;
export declare const generateHtmlDiffPreviewFile: (block: Block, diff: [HexString, HexString | null][], filename: string) => Promise<string>;
