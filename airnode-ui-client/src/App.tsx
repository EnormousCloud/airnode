import { useReducer } from 'react';
import { Link, HashRouter, Routes, Route, useParams } from 'react-router-dom';
import { Storage } from './service/Storage';
import { PersistentState } from './service/types';
import { MenuItem } from './components/Menu';
import { Select } from './screens/Select';
import { AddContract } from './screens/AddContract';

interface AppState {
  /// persistent part of the state
  nodes: PersistentState,
  /// current menu of the airnode (do not show the menu if absent)
  airnodeMenu: Array<MenuItem>,
  /// current menu of the selected rrp contract
  rrpMenu: Array<MenuItem>,
}

const defaultState: AppState  = {
  nodes: {
    filters: [],
  },
  airnodeMenu: [],
  rrpMenu: [],
};

function reducer(state: any, action: any) {
  switch (action.type) {
    case 'CHANGE_FILTER_FIELD':
      // return Storage.set({});
      return state;
    default:
      throw new Error();
  }
}


const App = () => {
  const [state, dispatch] = useReducer(reducer, defaultState, (s: any) => {
    return { ...defaultState, selected: Storage.get(s.selected) }
  });
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Select />} />
        <Route path="/add" element={<AddContract />} />
      </Routes>
    </HashRouter>
  )
}

export default App
