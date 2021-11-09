import { useReducer, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { API } from './service/Api';
import { defaultState, initState, reducer } from './service/AppState';
import { ErrorScreen } from './components/ErrorScreen';
import { LoadingScreen } from './screens/LoadingScreen';
import { ChangeFilter } from './screens/ChangeFilter';
import { Select } from './screens/Select';
import { AddContract } from './screens/AddContract';
import { RrpAirnodes } from './screens/RrpAirnodes';
import { RrpOperations } from './screens/RrpOperations';
import { RrpAdmins } from './screens/RrpAdmins';
import { AirnodeRequests } from './screens/AirnodeRequests';
import { AirnodeOperations } from './screens/AirnodeOperations';
import { AirnodeEndpoints } from './screens/AirnodeEndpoints';
import { AirnodeWhitelist } from './screens/AirnodeWhitelist';
import { AirnodeWithdrawals } from './screens/AirnodeWithdrawals';
import { DataIsLoading } from './service/types';

export const RRP: any = {};

const init = (type: string, payload: any): any => {
  const [state, dispatch] = useReducer(reducer, defaultState, initState);
  useEffect(() => {
    if (state.nodeStatus.isLoading) {
      API.fetchState().then((result: any) => {
        dispatch({ type: 'STATE_READY', payload: result });
        if (type) dispatch({ type, payload });
      }).catch((e: any) => {
        dispatch({ type: 'STATE_ERROR', payload: e });
      })
    }
  }, []);
  return [state, dispatch];
}

RRP.Select = () => {
  const [state, dispatch] = init('SELECT_NONE', {});
  const { nodeStatus } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  return <Select />
}

RRP.Add = () => {
  const [state, dispatch] = init('SELECT_NONE', {});
  const { nodeStatus } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  return <AddContract />
}

RRP.ChangeFilter = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  return <ChangeFilter {...{ menu, chainId, contractAddress }} />
};

RRP.GetProviders = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  return <RrpAirnodes {...{ menu, chainId, contractAddress }} />
}

RRP.GetOperations = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  const operations = new Array(); // TODO: load operations
  const dataStatus = DataIsLoading;
  return <RrpOperations {...{ menu, chainId, contractAddress, dataStatus, operations }} />
}

RRP.GetAdmins = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  return <RrpAdmins {...{ menu, chainId, contractAddress }} />
}

export const Airnode: any = {};

Airnode.GetRequests = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;

  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />

  // TODO: pubkey - from the state of the airnode
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return <AirnodeRequests {...{ menu, chainId, contractAddress, provider, xPubKey }} />
}

Airnode.GetOperations = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  const operations = new Array(); // TODO: load operations
  const dataStatus = DataIsLoading;
  return <AirnodeOperations {...{ menu, chainId, contractAddress, provider, dataStatus, operations }} />
}

Airnode.GetEndpoints = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  // TODO: pubkey - from the state of the airnode
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return <AirnodeEndpoints {...{ menu, chainId, contractAddress, provider, xPubKey }} />
}

Airnode.GetWhitelist = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  // TODO: pubkey - from the state of the airnode
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return <AirnodeWhitelist {...{ menu, chainId, contractAddress, provider, xPubKey }} />
}

Airnode.GetWithdrawals = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { nodeStatus, menu } = state;
  if (nodeStatus.isLoading) return <LoadingScreen />;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />

  // TODO: pubkey - from the state of the airnode
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return <AirnodeWithdrawals {...{ menu, chainId, contractAddress, provider, xPubKey }} />
}