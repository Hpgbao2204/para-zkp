import type { DigestItem, Header, TransactionValidityError } from '@polkadot/types/interfaces';
import type { HexString } from '@polkadot/util/types';
import type { TaskCallResponse } from '../wasm-executor/index.js';
import { Block } from './block.js';
import type { InherentProvider } from './inherent/index.js';
import type { BuildBlockParams } from './txpool.js';
export declare const genesisDigestLogs: (head: Block) => Promise<DigestItem[]>;
export declare const newHeader: (head: Block, unsafeBlockHeight?: number) => Promise<Header>;
export type BuildBlockCallbacks = {
    onApplyExtrinsicError?: (extrinsic: HexString, error: TransactionValidityError) => void;
    onPhaseApplied?: (phase: 'initialize' | 'finalize' | number, resp: TaskCallResponse) => void;
};
export declare const buildBlock: (head: Block, inherentProviders: InherentProvider[], params: BuildBlockParams, callbacks?: BuildBlockCallbacks) => Promise<[Block, HexString[]]>;
export declare const dryRunExtrinsic: (head: Block, inherentProviders: InherentProvider[], extrinsic: HexString | {
    call: HexString;
    address: string;
}, params: BuildBlockParams) => Promise<TaskCallResponse>;
export declare const dryRunInherents: (head: Block, inherentProviders: InherentProvider[], params: BuildBlockParams) => Promise<[HexString, HexString | null][]>;
