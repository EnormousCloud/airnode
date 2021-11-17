import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { ProvidersList } from "../components/ProvidersList";
import { getChainName } from '../service/Chain';

interface RrpAirnodesProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    nodeState: any
}

export const RrpAirnodes = (props: RrpAirnodesProps) => {
    const { chainId, contractAddress } = props;
    const { nodeState } = props;
    const providers = nodeState.providers || nodeState.airnodes;
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Airnodes</h1>
                        <div style={{ textAlign: 'center', fontSize: '0.8rem' }} className="darken">
                            {getChainName(chainId)} Contract &nbsp;
                            <strong>{contractAddress}</strong>
                        </div>
                        <ProvidersList
                            providers={providers}
                            chainId={chainId}
                            contractAddress={contractAddress}
                        />
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};