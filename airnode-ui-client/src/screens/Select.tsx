import { Link } from "react-router-dom";
import { Footer } from "../components/Footer";
import { AddressLink } from "../components/forms/EtherscanLink";
import { getChainName } from "./../service/Chain";

interface ScreenSelectProps {
  fullState: Array<any>;
}

const shortened = (x: string, ln: number) => {
  if (!x || !x.startsWith("0x") || x.length <= ln * 2 + 2) return x;
  return x.substring(0, ln + 2) + "..." + x.substring(x.length - ln);
};

export const Select = (props: ScreenSelectProps) => {
  const { fullState } = props;
  return (
    <div className="no-airnode">
      <main>
        <div className="inner" style={{ minHeight: "auto" }}>
          <div className="content">
            <h1>SELECT AIRNODE</h1>
            {fullState.map((node: any) => {
              const rrpURL = "/" + node.chain_id + "/" + node.contract_address;
              const providers = node.providers;
              return (
                <div className="well" key={node.contract_address}>
                  <h3 title="RRP Contract">
                    <strong>{getChainName(node.chain_id)}</strong>
                    &nbsp;
                    <AddressLink
                      chainId={node.chain_id + ""}
                      address={node.contract_address}
                    />
                    ({node.operations_num} operations)
                  </h3>
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
                            const airnodeURL =
                              rrpURL + "/nodes/" + p + "/operations";
                            const requests = Object.keys(
                              airnode.requests
                            ).length;
                            const endpoints = Object.keys(
                              airnode.endpoints
                            ).length;
                            const functions = Object.keys(
                              airnode.functions
                            ).length;
                            const templates = Object.keys(
                              airnode.templates
                            ).length;
                            return (
                              <tr key={p}>
                                <td>
                                  <Link to={airnodeURL}>
                                    {shortened(p, 16)}
                                  </Link>
                                </td>
                                <td className="stats">
                                  {requests ? requests : "-"}
                                </td>
                                <td className="stats">
                                  {endpoints ? endpoints : "-"}
                                </td>
                                <td className="stats">
                                  {functions ? functions : "-"}
                                </td>
                                <td className="stats">
                                  {templates ? templates : "-"}
                                </td>
                              </tr>
                            );
                          })}
                      </tbody>
                    </table>
                  ) : null}

                  {providers ? (
                    <ol className="mobile-only">
                      {Object.keys(providers)
                        .filter((p: any) => providers[p].operations_num > 0)
                        .map((p: any) => {
                          const airnode = providers[p];
                          const airnodeURL =
                            rrpURL + "/nodes/" + p + "/operations";
                          const requests = Object.keys(airnode.requests).length;
                          const endpoints = Object.keys(
                            airnode.endpoints
                          ).length;
                          const functions = Object.keys(
                            airnode.functions
                          ).length;
                          const templates = Object.keys(
                            airnode.templates
                          ).length;
                          return (
                            <li key={p}>
                              <div className="">
                                <Link to={airnodeURL}>{shortened(p, 8)}</Link>
                              </div>
                              <div className="row-stats">
                                <div className="stats">
                                  <span className="darken">Operations:</span>
                                  <span>
                                    {node.providers[p].operations_num}
                                  </span>
                                </div>
                                {requests ? (
                                  <div className="stats">
                                    <span className="darken">Requests:</span>
                                    <span>{requests ? requests : "-"}</span>
                                  </div>
                                ) : null}
                                {endpoints ? (
                                  <div className="stats">
                                    <span className="darken">Endpoints:</span>
                                    <span>{endpoints ? endpoints : "-"}</span>
                                  </div>
                                ) : null}
                                {functions ? (
                                  <div className="stats">
                                    <span className="darken">Functions:</span>
                                    <span>{functions ? functions : "-"}</span>
                                  </div>
                                ) : null}
                                {templates ? (
                                  <div className="stats">
                                    <span className="darken">Templates:</span>
                                    <span>{templates ? templates : "-"}</span>
                                  </div>
                                ) : null}
                              </div>
                            </li>
                          );
                        })}
                    </ol>
                  ) : null}
                  <hr />
                </div>
              );
            })}
          </div>
        </div>
      </main>
      <div className="desktop-only" style={{ height: 100 }}></div>
      <Footer />
    </div>
  );
};
