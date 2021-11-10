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
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
  return <pre>{JSON.stringify(state, null, 2)}</pre>
  return <Select />
}

RRP.Add = () => {
  const [state, dispatch] = init('SELECT_NONE', {});
  const { nodeStatus } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
  return <AddContract />
}

RRP.ChangeFilter = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
  return <ChangeFilter {...{ menu, chainId, contractAddress }} />
};

RRP.GetProviders = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
  return <RrpAirnodes {...{ menu, chainId, contractAddress }} />
}

RRP.GetOperations = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const [state, dispatch] = init('SELECT_RRP', { chainId, contractAddress });
  const { nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
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
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading) return <LoadingScreen />;
  return <RrpAdmins {...{ menu, chainId, contractAddress }} />
}

export const Airnode: any = {};

Airnode.GetRequests = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { airnodeState, nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading || !airnodeState) return <LoadingScreen />;
  return <AirnodeRequests {...{ menu, chainId, contractAddress, provider, airnodeState }} />
}

Airnode.GetOperations = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { airnodeState, nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading || !airnodeState) return <LoadingScreen />;
  const operations = new Array(); // TODO: load operations
  const dataStatus = DataIsLoading;
  return <AirnodeOperations {...{ menu, chainId, contractAddress, provider, dataStatus, airnodeState, operations }} />
}

Airnode.GetEndpoints = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { airnodeState, nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading || !airnodeState) return <LoadingScreen />;
  return <AirnodeEndpoints {...{ menu, chainId, contractAddress, provider, airnodeState }} />
}

Airnode.GetWhitelist = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { airnodeState, nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading || !airnodeState) return <LoadingScreen />;
  return <AirnodeWhitelist {...{ menu, chainId, contractAddress, provider, airnodeState }} />
}

Airnode.GetWithdrawals = () => {
  const params = useParams();
  const chainId = parseInt(params.chainId as string);
  const contractAddress = params.contractAddress as string;
  const provider = params.provider as string;
  const [state, dispatch] = init('SELECT_AIRNODE', { chainId, contractAddress, provider });
  const { airnodeState, nodeStatus, menu } = state;
  if (nodeStatus.errorMessage) return <ErrorScreen error={nodeStatus.errorMessage} />
  if (nodeStatus.isLoading || !airnodeState) return <LoadingScreen />;
  return <AirnodeWithdrawals {...{ menu, chainId, contractAddress, provider, airnodeState }} />
}