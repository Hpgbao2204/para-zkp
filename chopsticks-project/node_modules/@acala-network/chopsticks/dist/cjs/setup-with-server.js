"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "setupWithServer", {
    enumerable: true,
    get: function() {
        return setupWithServer;
    }
});
const _chopstickscore = require("@acala-network/chopsticks-core");
const _context = require("./context.js");
const _index = require("./rpc/index.js");
const _server = require("./server.js");
const setupWithServer = async (argv)=>{
    if (argv.addr) {
        _chopstickscore.defaultLogger.warn({}, `⚠️ Option --addr is deprecated, please use --host instead.`);
        argv.host ??= argv.addr;
    }
    const context = await (0, _context.setupContext)(argv);
    const { close, addr } = await (0, _server.createServer)((0, _index.handler)(context), argv.port, argv.host);
    _chopstickscore.defaultLogger.info(`${await context.chain.api.getSystemChain()} RPC listening on http://${addr} and ws://${addr}`);
    return {
        ...context,
        addr,
        async close () {
            await context.chain.close();
            await context.fetchStorageWorker?.terminate();
            await close();
        }
    };
};
