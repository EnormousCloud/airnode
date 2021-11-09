import { HashRouter, Routes, Route } from 'react-router-dom';
import { Airnode, RRP } from './Routes';

const App = () => {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<RRP.Select />} />
        <Route path="/add/:chainId" element={<RRP.Add />} />
        <Route path="/:chainId/:contractAddress/filter" element={<RRP.ChangeFilter />} />
        <Route path="/:chainId/:contractAddress/airnodes" element={<RRP.GetProviders />} />
        <Route path="/:chainId/:contractAddress/operations" element={<RRP.GetOperations />} />
        <Route path="/:chainId/:contractAddress/admins" element={<RRP.GetAdmins />} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/requests" element={<Airnode.GetRequests />}/>
        <Route path="/:chainId/:contractAddress/nodes/:provider/operations" element={<Airnode.GetOperations />} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/endpoints" element={<Airnode.GetEndpoints />} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/whitelist" element={<Airnode.GetWhitelist />} />
        <Route path="/:chainId/:contractAddress/nodes/:provider/withdrawals" element={<Airnode.GetWithdrawals />} />
      </Routes>
    </HashRouter>
  )
}

export default App
