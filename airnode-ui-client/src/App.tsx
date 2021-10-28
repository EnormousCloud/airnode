import { useReducer } from 'react';
import { Link, HashRouter, Routes, Route, useParams } from 'react-router-dom';
import { Storage } from './service/Storage';
import { PersistentState } from './service/types';
import { MenuPanelProps } from './components/MenuPanel';
import { mockMenu } from "./fixtures/menu";
import { Select } from './screens/Select';
import { AddContract } from './screens/AddContract';
import { ChangeFilter } from './screens/ChangeFilter';

interface AppState {
  /// persistent part of the state
  nodes: PersistentState,
}

const defaultState: AppState  = {
  nodes: {
    filters: [],
  },
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
  const menu = mockMenu;
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Select />} />
        <Route path="/add" element={<AddContract />} />
        <Route path="/:chainId/:contractAddress/filter" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          return <ChangeFilter {...{ menu, chainId, contractAddress }} />
        }} />
      </Routes>
    </HashRouter>
  )
}

export default App
