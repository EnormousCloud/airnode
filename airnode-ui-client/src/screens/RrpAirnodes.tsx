import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface RrpAirnodesProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    nodeState: any
}

// const shortened = (x: string, ln: number) => {
//   if (!x || !x.startsWith("0x") || x.length <= ln * 2 + 2) return x;
//   return x.substring(0, ln + 2) + "..." + x.substring(x.length - ln);
// };

export const RrpAirnodes = (props: RrpAirnodesProps) => {
    const { chainId, contractAddress } = props;
    const { nodeState } = props;
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
                        <pre>{JSON.stringify(nodeState, null, 2)}</pre>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};