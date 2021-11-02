import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface RrpAirnodesProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const RrpAirnodes = (props: RrpAirnodesProps) => {
    const { chainId, contractAddress } = props;
    const provider = '';
    const xPubKey = '';
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Airnodes</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};