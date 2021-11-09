import { DataStatus, DataIsLoading, DataIsReady, AirnodeRef, PersistentState } from './types';
import { Storage } from './Storage';
import { noMenu } from "../fixtures/menu";
import { MenuItem } from '../components/Menu';
import { MenuPanelProps } from '../components/MenuPanel';

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
}

export const defaultState: AppState = {
    selected: null,
    nodeStatus: DataIsLoading,
    dataStatus: DataIsReady,
    nodes: {
        filters: [],
    },
    menu: noMenu,
};

export const initState = (s: AppState): AppState => {
    return { ...defaultState, nodes: Storage.get(s.nodes) }
}

export const niceError = (e: any) => {
    return e + '';
}

const airnodeMenuFromState = (state: AppState, selected: AirnodeRef): MenuPanelProps => {
    const itemsAirnode: Array<MenuItem> = [
        { name: "Requests", href: "/" },
        { name: "Operations", href: "/" },
        { name: "Endpoints", href: "/" },
        { name: "Whitelist", href: "/" },
        { name: "Withdrawals", href: "/" },
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
        { name: "Operations", href: "/" },
        { name: "Admins", href: "/" },
    ];
    const rrp = { 
        title: 'RRP Contract',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/requests',
        address: selected.contractAddress,
        items: itemsRRP,
        balance: '',
        symbol: 'ETH',
    };
    return { airnode, rrp };
}

const rrpMenuFromState = (state: AppState, selected: AirnodeRef): MenuPanelProps => {
    const itemsRRP: Array<MenuItem> = [
        { name: "Operations", href: "/" },
        { name: "Admins", href: "/" },
    ];
    const rrp = { 
        title: 'RRP Contract',
        link: '/' + selected.chainId + '/' + selected.contractAddress + '/requests',
        address: selected.contractAddress,
        items: itemsRRP,
        balance: '',
        symbol: 'ETH',
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
            const menu = rrpMenuFromState(state, selected);
            return { ...state, selected, menu };
        }
        case 'SELECT_AIRNODE': {
            const { chainId, contractAddress, provider } = action.payload;
            const selected: AirnodeRef = { chainId, contractAddress, provider }
            const menu = airnodeMenuFromState(state, selected);
            return { ...state, selected, menu };
        }
        case 'STATE_ERROR':
            return { ...state, nodeStatus: { isLoading: false, errorMessage: niceError(action.payload) } };
        case 'STATE_READY':
            return { ...state, nodeStatus: DataIsReady };
        case 'OPERATIONS_INIT':
            return { ...state, dataStatus: DataIsReady };
        case 'OPERATIONS_ERROR':
            return { ...state, dataStatus: { isLoading: false, errorMessage: niceError(action.payload) } };
        case 'OPERATIONS_READY':
            return { ...state, dataStatus: DataIsReady };
        default:
            throw new Error();
    }
}
