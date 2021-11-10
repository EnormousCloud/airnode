export interface NiceBalance {
    balance: string
    symbol: string
}

const niceBalance = (x: string, _decimals: number) => {
    if (x === '0x0') return '0';
    return x;
}

export const parseBalance = (obj: any): NiceBalance => {
    if (!obj || !obj.last_value || !obj.symbol) return { balance: '', symbol: '' };

    const symbol = obj.symbol as string;
    const decimals = obj.decimals as number;
    const balance = niceBalance(obj.last_value as string, decimals); // this 0x
    
    return { balance, symbol };
}