import { type Blockchain, type StorageValues } from '@acala-network/chopsticks-core';
import type { HexString } from '@polkadot/util/types';
export declare const overrideStorage: (chain: Blockchain, storage?: string | StorageValues, at?: HexString) => Promise<void>;
export declare const overrideWasm: (chain: Blockchain, wasmPath?: string, at?: HexString) => Promise<void>;
