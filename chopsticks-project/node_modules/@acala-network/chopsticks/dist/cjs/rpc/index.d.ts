import { type Context, type SubscriptionManager } from '@acala-network/chopsticks-core';
export declare const handler: (context: Context) => ({ method, params }: {
    method: string;
    params: any[];
}, subscriptionManager: SubscriptionManager) => Promise<any>;
