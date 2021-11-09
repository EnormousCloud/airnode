import { DataStatus, DataIsLoading, DataIsReady, PersistentState } from './types';
import { noMenu } from "../fixtures/menu";
import { MenuPanelProps } from '../components/MenuPanel';

export interface AppState {
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
    nodeStatus: DataIsLoading,
    dataStatus: DataIsReady,
    nodes: {
        filters: [],
    },
    menu: noMenu,
};

export const initState = (s: AppState): AppState => {
    // return { ...defaultState, selected: Storage.get(s.selected) }
    return { ...defaultState };
}

export const niceError = (e: any) => {
    return e + '';
}

export const reducer = (state: AppState, action: any): AppState => {
    switch (action.type) {
        case 'SELECT_NONE': {
            const menu = { rrp: null, airnode: null };
            return { ...state, menu };
        }
        case 'SELECT_RRP': {
            const { chainId, contractAddress} = action.payload;
            // TODO:
            const menu = { ...state.menu, airnode: null };
            return { ...state, menu };
        }
        case 'SELECT_AIRNODE': {
            const { chainId, contractAddress, provider } = action.payload;
            const airnode = { 
                title: 'Airnode',
                link: '',
                address: '',
                items: [], // <MenuItem>
                balance: '',
                symbol: '',
            };
            const menu = { ...state.menu, rrp: null, airnode };
            return { ...state, menu };
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
