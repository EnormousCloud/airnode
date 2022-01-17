
export const getChainName = (chainId: number): string => {
    if (chainId === 0) return "MAINNET"
    if (chainId === 3) return "ROPSTEN";
    if (chainId === 4) return "RINKEBY";
    if (chainId === 42) return "KOVAN";
    if (chainId === 30) return "RSK";
    if (chainId === 31) return "RSK TESTNET";
    if (chainId === 100) return "XDAI";
    return 'Chain ' + chainId;
};

export const getChainEtherscan = (chainId: string) => {
    if (chainId === "0") return "https://etherscan.io";
    if (chainId === "3") return "https://ropsten.etherscan.io";
    if (chainId === "4") return "https://rinkeby.etherscan.io";
    if (chainId === "42") return "https://kovan.etherscan.io";
    if (chainId === "30") return "https://explorer.rsk.co";
    if (chainId === "31") return "https://explorer.testnet.rsk.co";
    if (chainId === "100") return "https://blockscout.com/xdai/mainnet";
    return "";
};