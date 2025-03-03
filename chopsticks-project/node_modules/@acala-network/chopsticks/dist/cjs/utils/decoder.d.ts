import { type Block } from '@acala-network/chopsticks-core';
import type { HexString } from '@polkadot/util/types';
export declare const decodeStorageDiff: (block: Block, diff: [HexString, HexString | null][]) => Promise<{
    oldState: {};
    newState: {};
    delta: import("jsondiffpatch").Delta | undefined;
}>;
