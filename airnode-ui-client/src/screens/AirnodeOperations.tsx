import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { DataStatus } from './../service/types';

interface AirnodeOperationsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
    dataStatus: DataStatus
    airnodeState: any
    operations: Array<any>
}

export const AirnodeOperations = (props: AirnodeOperationsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey } = airnodeState;
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                        <pre>{JSON.stringify(airnodeState, null, 2)}</pre>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};