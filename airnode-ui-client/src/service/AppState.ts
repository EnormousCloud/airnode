import { DataStatus, DataIsLoading, DataIsReady, AirnodeRef, PersistentState } from './types';
import { Storage } from './Storage';
import { noMenu } from "../fixtures/menu";
import { MenuItem } from '../components/Menu';
import { MenuPanelProps } from '../components/MenuPanel';
import { parseBalance } from './Balance';

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
    operations: any
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

export const airnodeMenuFromState = (state: AppState, selected: AirnodeRef, op: string): MenuPanelProps => {
    const rrpState = state.fullState.find((x:any) => (
        x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
    ));
    const { providers, airnodes } = rrpState;
    const p = (airnodes && Object.keys(airnodes).length > 0) ? airnodes : providers;
    const airnodeState = p[selected.provider as string];
    const baseRRP = '/' + selected.chainId + '/' + selected.contractAddress;
    const baseURL = '/' + selected.chainId + '/' + selected.contractAddress + '/nodes/' + selected.provider;
    const withdrawals = Object.keys(airnodeState.requests).filter((req: string) => (
        !!airnodeState.requests[req].withdraw
    ));
    const itemsAirnode: Array<MenuItem> = [
        { name: "Requests", href: baseURL + '/requests', counter: Object.keys(airnodeState.requests).length, active: op === 'requests'}, 
        { name: "Operations", href: baseURL + '/operations', counter: airnodeState.operations_num, active: op === 'operations' },
        { name: "Endpoints", href: baseURL + '/endpoints', counter: Object.keys(airnodeState.endpoints).length, active: op === 'endpoints' },
        { name: "Whitelist", href: baseURL + '/whitelist', counter: airnodeState.whitelisted.length, active: op === 'whitelist' },
        { name: "Withdrawals", href: baseURL + '/withdrawals', counter: withdrawals.length, active: op === 'withdrawals' },
    ];
    const airnode = { 
        title: 'Airnode',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/nodes/' + selected.provider + '/operations',
        chainId: selected.chainId,
        address: selected.provider as string,
        items: itemsAirnode,
        ...parseBalance(airnodeState.balance),
    };
    const itemsRRP: Array<MenuItem> = [
        { name: "Operations", href: baseRRP + '/operations', counter: rrpState.operations_num },
        { name: "Admins", href: baseRRP + '/admins' },
    ];

    const rrp = { 
        title: 'RRP Contract',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/providers',
        chainId: selected.chainId,
        address: selected.contractAddress,
        items: itemsRRP,
        ...parseBalance(rrpState.balance)
    };
    return { airnode, rrp };
}

export const rrpMenuFromState = (state: AppState, selected: AirnodeRef, op: string): MenuPanelProps => {
    const baseRRP = '/' + selected.chainId + '/' + selected.contractAddress;
    const rrpState = state.fullState.find((x:any) => (
        x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
    ));
    const itemsRRP: Array<MenuItem> = [
        { name: "Operations", href: baseRRP + '/operations', counter: rrpState.operations_num, active: op === 'operations' },
        { name: "Admins", href: baseRRP + '/admins', active: op === 'admins' },
    ];
    const rrp = { 
        title: 'RRP Contract',
        link: baseRRP + '/providers',
        chainId: selected.chainId,
        address: selected.contractAddress,
        items: itemsRRP,
        ...parseBalance(rrpState.balance),
    };
    return { airnode: null, rrp };
}

export const reducer = (state: AppState, action: any): AppState => {
    // console.log('reduce', action.type, JSON.stringify(Object.keys(action.payload)));
    switch (action.type) {
        case 'SELECT_NONE': {
            const menu = { rrp: null, airnode: null };
            return { ...state, menu, selected: null };
        }
        case 'SELECT_RRP': {
            const { chainId, contractAddress, activeMenu } = action.payload;
            const provider = '';
            const selected: AirnodeRef = { chainId, contractAddress, provider }
            const rrpState = state.fullState.find((x:any) => (
                x.chain_id == selected.chainId && x.contract_address == selected.contractAddress
            ));
            if (!rrpState) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' not found in network ' + selected.chainId;
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const menu = rrpMenuFromState(state, selected, activeMenu);
            return { ...state, selected, airnodeState: null, menu };
        }
        case 'SELECT_AIRNODE': {
            const { chainId, contractAddress, provider, activeMenu } = action.payload;
            const selected: AirnodeRef = { chainId, contractAddress, provider }
            const rrpState = state.fullState.find((x:any) => (
                parseInt(x.chain_id) == selected.chainId && x.contract_address == selected.contractAddress
            ));
            if (!rrpState) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' not found in network ' + selected.chainId;
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const { providers, airnodes } = rrpState;
            const p = (airnodes && Object.keys(airnodes).length > 0) ? airnodes : providers;
            if (!p) {
                const errorMessage = 'RRP contract ' + selected.contractAddress + ' in network ' + selected.chainId + ' has no airnodes';
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const airnodeState = p[selected.provider as string];
            if (!airnodeState) {
                const errorMessage = 'Provider ' + selected.provider + ' is not found in RRP contract ' + selected.contractAddress + ' in network ' + selected.chainId;
                return { ...state, nodeStatus: { isLoading: false, errorMessage } };
            }
            const menu = airnodeMenuFromState(state, selected, activeMenu);
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
            const key = state.selected?.chainId + "-" + state.selected?.contractAddress;
            operations[ key ] = action.payload;
            return { ...state, dataStatus: DataIsReady, operations };
        default:
            throw new Error();
    }
}
