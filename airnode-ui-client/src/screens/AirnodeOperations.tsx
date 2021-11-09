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
    operations: Array<any>
}

export const AirnodeOperations = (props: AirnodeOperationsProps) => {
    const { chainId, contractAddress, provider } = props;
    // todo: 
    const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
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