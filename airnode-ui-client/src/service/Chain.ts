
export const getChainName = (chainId: number): string => {
    if (chainId === 0) return "MAINNET"
    if (chainId === 4) return "RINKEBY";
    if (chainId === 42) return "KOVAN";
    if (chainId === 30) return "RSK";
    if (chainId === 31) return "RSK TESTNET";
    if (chainId === 100) return "XDAI";
    return 'Chain ' + chainId;
};