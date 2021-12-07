import { Footer } from "../components/Footer";
import { AddressLink } from "../components/forms/EtherscanLink";
import { getChainName } from "./../service/Chain";
import { ProvidersList } from "../components/ProvidersList";

interface ScreenSelectProps {
  fullState: Array<any>;
}

export const Select = (props: ScreenSelectProps) => {
  const { fullState } = props;
  return (
    <div className="no-airnode">
      <main>
        <div className="inner" style={{ minHeight: "auto" }}>
          <div className="content">
            <h1>SELECT AIRNODE</h1>
            {fullState.map((node: any) => {
              const { providers, airnodes } = node;
              const p = (airnodes && Object.keys(airnodes).length > 0) ? airnodes : providers;
              return (
                <div className="well" key={node.contract_address}>
                  <h3 title="RRP Contract">
                    <strong>{getChainName(node.chain_id)}</strong>
                    &nbsp;
                    <AddressLink
                      chainId={node.chain_id + ""}
                      address={node.contract_address}
                    />
                    <br className="mobile-only" />
                    <small>({node.operations_num} operations)</small>
                  </h3>
                  <ProvidersList
                    providers={p}
                    chainId={node.chain_id}
                    contractAddress={node.contract_address}
                  />
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
