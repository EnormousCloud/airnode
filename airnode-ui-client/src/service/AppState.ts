import { DataStatus, DataIsLoading, DataIsReady, AirnodeRef, PersistentState } from './types';
import { Storage } from './Storage';
import { noMenu } from "../fixtures/menu";
import { MenuItem } from '../components/Menu';
import { MenuPanelProps } from '../components/MenuPanel';
import { parseBalance, NiceBalance } from './Balance';

export interface AppState {
    // selection in the menu
    selected: AirnodeRef|null
    // persistent part of the state
    nodes: PersistentState
    // status of the nodes state
    nodeStatus: DataStatus
    // status of the data (operations)
    dataStatus: DataStatus
    // menu with counters
    menu: MenuPanelProps
    // state of the currently selected airnode (if any)
    airnodeState: any
    // full state, downloaded
    fullState: Array<any>
    // list of operations for each node, downloaded
    operations: Map<string, Array<any>>
}

export const defaultState: AppState = {
    selected: null,
    nodeStatus: DataIsLoading,
    dataStatus: DataIsReady,
    nodes: {
        filters: [],
    },
    menu: noMenu,
    airnodeState: null,
    fullState: [],
    operations: new Map<string, Array<any>>(),
};

export const initState = (s: AppState): AppState => {
    return { ...defaultState, nodes: Storage.get(s.nodes) }
}

export const niceError = (e: any) => {
    return e + '';
}

const airnodeMenuFromState = (state: AppState, selected: AirnodeRef): MenuPanelProps => {
    const rrpState = state.fullState.find((x:any) => (
        x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
    ));
    const baseRRP = '/' + selected.chainId + '/' + selected.contractAddress;
    const baseURL = '/' + selected.chainId + '/' + selected.contractAddress + '/nodes/' + selected.provider;
    const itemsAirnode: Array<MenuItem> = [
        { name: "Requests", href: baseURL + '/requests'},
        { name: "Operations", href: baseURL + '/operations' },
        { name: "Endpoints", href: baseURL + '/endpoints' },
        { name: "Whitelist", href: baseURL + '/whitelist' },
        { name: "Withdrawals", href: baseURL + '/withdrawals' },
    ];
    const airnode = { 
        title: 'Airnode',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/nodes/' + selected.provider + '/requests',
        address: selected.provider as string,
        items: itemsAirnode,
        balance: '',
        symbol: 'ETH',
    };
    const itemsRRP: Array<MenuItem> = [
        { name: "Operations", href: baseRRP + '/operations', counter: rrpState.operations_num },
        { name: "Admins", href: baseRRP + '/admins' },
    ];

    const { balance, symbol } = parseBalance(rrpState.balance);
    const rrp = { 
        title: 'RRP Contract',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/requests',
        address: selected.contractAddress,
        items: itemsRRP,
        balance,
        symbol,
    };
    return { airnode, rrp };
}

const rrpMenuFromState = (state: AppState, selected: AirnodeRef): MenuPanelProps => {
    const baseRRP = '/' + selected.chainId + '/' + selected.contractAddress;
    const rrpState = state.fullState.find((x:any) => (
        x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
    ));
    const itemsRRP: Array<MenuItem> = [
        { name: "Operations", href: baseRRP + '/operations', counter: rrpState.operations_num },
        { name: "Admins", href: baseRRP + '/admins' },
    ];
    const { balance, symbol } = parseBalance(rrpState.balance);
    const rrp = { 
        title: 'RRP Contract',
        link: baseRRP + '/requests',
        address: selected.contractAddress,
        items: itemsRRP,
        balance,
        symbol,
    };
    return { airnode: null, rrp };
}

export const reducer = (state: AppState, action: any): AppState => {
    console.log('reduce', action.type, JSON.stringify(Object.keys(action.payload)));
    switch (action.type) {
        case 'SELECT_NONE': {
            const menu = { rrp: null, airnode: null };
            return { ...state, menu, selected: null };
        }
        case 'SELECT_RRP': {
            const { chainId, contractAddress } = action.payload;
            const provider = '';
            const selected: AirnodeRef = { chainId, contractAddress, provider }
            const rrpState = state.fullState.find((x:any) => (
                x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
            ));
            if (!rrpState) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' not found in network ' + selected.chainId;
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const menu = rrpMenuFromState(state, selected);
            return { ...state, selected, airnodeState: null, menu };
        }
        case 'SELECT_AIRNODE': {
            const { chainId, contractAddress, provider } = action.payload;
            const selected: AirnodeRef = { chainId, contractAddress, provider }
            const rrpState = state.fullState.find((x:any) => (
                parseInt(x.chain_id) == selected.chainId && x.contract_address == selected.contractAddress
            ));
            state.fullState.forEach(x => {
                console.log(x.chain_id, selected.chainId, parseInt(x.chainId) == selected.chainId, x.contract_address == selected.contractAddress);
            });

            if (!rrpState) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' not found in network ' + selected.chainId;
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            if (!rrpState.providers && !rrpState.airnodes) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' in network ' + selected.chainId + ' has no airnodes';
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const airnodeState = rrpState.providers 
                ? rrpState.providers[selected.provider as string]
                : rrpState.airnodes[selected.provider as string];
            const menu = airnodeMenuFromState(state, selected);
            return { ...state, selected, airnodeState, menu };
        }
        case 'STATE_ERROR':
            return { ...state, nodeStatus: { isLoading: false, errorMessage: niceError(action.payload) } };
        case 'STATE_READY':
            return { ...state, nodeStatus: DataIsReady, fullState: action.payload };
        case 'OPERATIONS_INIT':
            return { ...state, dataStatus: DataIsLoading };
        case 'OPERATIONS_ERROR':
            return { ...state, dataStatus: { isLoading: false, errorMessage: niceError(action.payload) } };
        case 'OPERATIONS_READY':
            const operations = { ...state.operations };
            const key = state.selected?.chainId + "-" + state.selected?.contractAddress + "-" + state.selected?.provider;
            operations.set(key, action.payload);
            return { ...state, dataStatus: DataIsReady, operations };
        default:
            throw new Error();
    }
}
