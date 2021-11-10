import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeRequestsProps {
    chainId: number
    contractAddress: string
    provider: string
    airnodeState: any
    menu: MenuPanelProps
}

export const AirnodeRequests = (props: AirnodeRequestsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey } = airnodeState;
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Requests</h1>
                        <pre>{JSON.stringify(airnodeState, null, 2)}</pre>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};