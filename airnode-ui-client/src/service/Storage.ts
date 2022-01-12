import { Debounced } from './Debounced';
import { PersistentState } from './types';

const key = "airnodes";
export const Storage = {
    get: (state: PersistentState): PersistentState => {
        try {
            const stateStr = sessionStorage.getItem(key);
            return stateStr ? JSON.parse(stateStr) : state;
        } catch {
        }
        return state;
    },
    set: (val: PersistentState) => {
        Debounced.start('key', () => {
            sessionStorage.setItem(key, JSON.stringify(val))
        }, 50);
        return val;
    }
};