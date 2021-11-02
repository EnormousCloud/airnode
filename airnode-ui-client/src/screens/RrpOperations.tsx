import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface RrpOperationsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const RrpOperations = (props: RrpOperationsProps) => {
    const { chainId, contractAddress } = props;
    const provider = '';
    const xPubKey = '';
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};