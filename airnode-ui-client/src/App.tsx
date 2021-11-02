import { useReducer } from 'react';
import { HashRouter, Routes, Route, useParams } from 'react-router-dom';
import { Storage } from './service/Storage';
import { PersistentState } from './service/types';
import { mockMenu } from "./fixtures/menu";
import { Select } from './screens/Select';
import { AddContract } from './screens/AddContract';
import { ChangeFilter } from './screens/ChangeFilter';
import { RrpAirnodes } from './screens/RrpAirnodes';
import { RrpOperations } from './screens/RrpOperations';
import { RrpAdmins } from './screens/RrpAdmins';
import { AirnodeRequests } from './screens/AirnodeRequests';
import { AirnodeOperations } from './screens/AirnodeOperations';
import { AirnodeEndpoints } from './screens/AirnodeEndpoints';
import { AirnodeWhitelist } from './screens/AirnodeWhitelist';
import { AirnodeWithdrawals } from './screens/AirnodeWithdrawals';

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
        <Route path="/:chainId/:contractAddress/airnodes" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          return <RrpAirnodes {...{ menu, chainId, contractAddress }} />
        }} />
        <Route path="/:chainId/:contractAddress/operations" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          return <RrpOperations {...{ menu, chainId, contractAddress }} />
        }} />
        <Route path="/:chainId/:contractAddress/admins" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          return <RrpAdmins {...{ menu, chainId, contractAddress }} />
        }} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/requests" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          const provider = params.provider as string;
          // TODO: pubkey - from the state of the airnode
          const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
          return <AirnodeRequests {...{ menu, chainId, contractAddress, provider, xPubKey }} />
        }} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/operations" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          const provider = params.provider as string;
          // TODO: pubkey - from the state of the airnode
          const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
          return <AirnodeOperations {...{ menu, chainId, contractAddress, provider, xPubKey }} />
        }} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/endpoints" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          const provider = params.provider as string;
          // TODO: pubkey - from the state of the airnode
          const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
          return <AirnodeEndpoints {...{ menu, chainId, contractAddress, provider, xPubKey }} />
        }} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/whitelist" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          const provider = params.provider as string;
          // TODO: pubkey - from the state of the airnode
          const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
          return <AirnodeWhitelist {...{ menu, chainId, contractAddress, provider, xPubKey }} />
        }} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/withdrawals" element={() => {
          const params = useParams();
          const chainId = parseInt(params.chainId as string);
          const contractAddress = params.contractAddress as string;
          const provider = params.provider as string;
          // TODO: pubkey - from the state of the airnode
          const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
          return <AirnodeWithdrawals {...{ menu, chainId, contractAddress, provider, xPubKey }} />
        }} />
      </Routes>
    </HashRouter>
  )
}

export default App
