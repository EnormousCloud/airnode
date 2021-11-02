import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeRequestsProps {
    chainId: number
    contractAddress: string
    provider: string
    xPubKey: string
    menu: MenuPanelProps
}

export const AirnodeRequests = (props: AirnodeRequestsProps) => {
    const { chainId, contractAddress, provider, xPubKey } = props;
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Requests</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};