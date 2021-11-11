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
                  {node.providers ? (
                    <ol>
                      {Object.keys(node.providers)
                        .filter(
                          (p: any) => node.providers[p].operations_num > 0
                        )
                        .map((p: any) => {
                          const airnodeURL =
                            rrpURL + "/nodes/" + p + "/operations";
                          return (
                            <li key={p}>
                              <Link to={airnodeURL}>{shortened(p, 8)}</Link>
                              <br />
                              <span className="darken">Operations:</span> 
                              <span>{node.providers[p].operations_num}</span>
                              &nbsp;
                              <span className="darken">Requests:</span> 
                              <span>{Object.keys(node.providers[p].requests).length}</span>
                              &nbsp;
                              <span className="darken">Endpoints:</span> 
                              <span>{Object.keys(node.providers[p].endpoints).length}</span>
                              &nbsp;
                              <span className="darken">Functions:</span> 
                              <span>{Object.keys(node.providers[p].functions).length}</span>
                              &nbsp;
                              <span className="darken">Templates:</span> 
                              <span>{Object.keys(node.providers[p].templates).length}</span>
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
