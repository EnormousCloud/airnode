import { Debounced } from './Debounced';
import { PeristentState } from './types';

const key = "airnodes";
export const MdmStorage = {
    get: (state: PeristentState): PeristentState => {
        try {
            const stateStr = sessionStorage.getItem(key);
            return stateStr ? JSON.parse(stateStr) : state;
        } catch {
        }
        return state;
    },
    set: (val: PeristentState) => {
        Debounced.start('key', () => {
            sessionStorage.setItem(key, JSON.stringify(val))
        }, 50);
        return val;
    }
};