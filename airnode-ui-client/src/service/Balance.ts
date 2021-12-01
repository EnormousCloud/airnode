import { utils } from 'ethers';

export interface NiceBalance {
    balance: string
    symbol: string
}

const pad6 = (x: string): string => {
    // this is far from perfect, we decided to wait for uiValue from the server side
    const parts = utils.formatEther(x).split(".");
    if (parts.length != 2) return x;
    if (parts[1].length > 6) return x;
    return parts[0] + '.' + parts[1].substring(0, 6);
}

const niceBalance = (x: string, _decimals: number) => {
    if (x === '0x0') return '0';
    return pad6(x);
}

export const parseBalance = (obj: any): NiceBalance => {
    if (!obj || !obj.last_value || !obj.symbol) return { balance: '', symbol: '' };

    const symbol = obj.symbol as string;
    const decimals = obj.decimals as number;
    const balance = niceBalance(obj.last_value as string, decimals); // this 0x
    
    return { balance, symbol };
}