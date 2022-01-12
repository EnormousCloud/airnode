import { Link } from "react-router-dom";

import './ProvidersList.css';

const shortened = (x: string, ln: number) => {
    if (!x || !x.startsWith("0x") || x.length <= ln * 2 + 2) return x;
    return x.substring(0, ln + 2) + "..." + x.substring(x.length - ln);
};
  
export interface ProvidersListProps {
  providers: any
  chainId: number
  contractAddress: string
}

export const ProvidersList = (props: ProvidersListProps) => {
  const { providers, chainId, contractAddress } = props;
  const rrpURL = "/" + chainId + "/" + contractAddress;
  return (
    <div>
      {providers ? (
        <table className="table table-providers desktop-only">
          <thead>
            <tr>
              <th>Provider</th>
              <th>Requests</th>
              <th>Endpoints</th>
              <th>Functions</th>
              <th>Templates</th>
            </tr>
          </thead>
          <tbody>
            {Object.keys(providers)
              .filter((p: any) => providers[p].operations_num > 0)
              .map((p: any) => {
                const airnode = providers[p];
                const airnodeURL = rrpURL + "/nodes/" + p + "/operations";
                const requests = Object.keys(airnode.requests).length;
                const endpoints = Object.keys(airnode.endpoints).length;
                const functions = Object.keys(airnode.functions).length;
                const templates = Object.keys(airnode.templates).length;
                return (
                  <tr key={p}>
                    <td>
                      <Link to={airnodeURL}>{shortened(p, 16)}</Link>
                    </td>
                    <td className="stats">{requests ? requests : "-"}</td>
                    <td className="stats">{endpoints ? endpoints : "-"}</td>
                    <td className="stats">{functions ? functions : "-"}</td>
                    <td className="stats">{templates ? templates : "-"}</td>
                  </tr>
                );
              })}
          </tbody>
        </table>
      ) : null}

      {providers ? (
        <ol className="list-providers mobile-only">
          {Object.keys(providers)
            .filter((p: any) => providers[p].operations_num > 0)
            .map((p: any) => {
              const airnode = providers[p];
              const airnodeURL = rrpURL + "/nodes/" + p + "/operations";
              const requests = Object.keys(airnode.requests).length;
              const endpoints = Object.keys(airnode.endpoints).length;
              const functions = Object.keys(airnode.functions).length;
              const templates = Object.keys(airnode.templates).length;
              return (
                <li key={p}>
                  <div className="">
                    <Link to={airnodeURL}>{shortened(p, 8)}</Link>
                  </div>
                  <div className="row-stats">
                    <div className="stats">
                      <span className="darken">Operations:</span>
                      <strong>{providers[p].operations_num}</strong>
                    </div>
                    {requests ? (
                      <div className="stats">
                        <span className="darken">Requests:</span>
                        <strong>{requests ? requests : "-"}</strong>
                      </div>
                    ) : null}
                    {endpoints ? (
                      <div className="stats">
                        <span className="darken">Endpoints:</span>
                        <strong>{endpoints ? endpoints : "-"}</strong>
                      </div>
                    ) : null}
                    {functions ? (
                      <div className="stats">
                        <span className="darken">Functions:</span>
                        <strong>{functions ? functions : "-"}</strong>
                      </div>
                    ) : null}
                    {templates ? (
                      <div className="stats">
                        <span className="darken">Templates:</span>
                        <strong>{templates ? templates : "-"}</strong>
                      </div>
                    ) : null}
                  </div>
                </li>
              );
            })}
        </ol>
      ) : null}
    </div>
  );
};
