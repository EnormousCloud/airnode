import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeEndpointsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
    airnodeState: any
}

export const AirnodeEndpoints = (props: AirnodeEndpointsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey } = airnodeState;
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Endpoints</h1>
                        <pre>{JSON.stringify(airnodeState.endpoints, null, 2)}</pre>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};