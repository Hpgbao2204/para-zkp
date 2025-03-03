import type { BlockEntry, Database, KeyValueEntry } from '@acala-network/chopsticks-core';
import type { HexString } from '@polkadot/util/types';
import type { DataSource } from 'typeorm';
export declare abstract class BaseSqlDatabase implements Database {
    abstract datasource: Promise<DataSource>;
    close: () => Promise<void>;
    saveBlock(block: BlockEntry): Promise<void>;
    queryBlock(hash: HexString): Promise<BlockEntry | null>;
    queryBlockByNumber(number: number): Promise<BlockEntry | null>;
    queryHighestBlock(): Promise<BlockEntry | null>;
    deleteBlock(hash: HexString): Promise<void>;
    blocksCount(): Promise<number>;
    saveStorage(blockHash: HexString, key: HexString, value: HexString | null): Promise<void>;
    saveStorageBatch(entries: KeyValueEntry[]): Promise<void>;
    queryStorage(blockHash: HexString, key: HexString): Promise<KeyValueEntry | null>;
}
