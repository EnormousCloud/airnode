import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { DataStatus } from './../service/types';

interface RrpOperationsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    dataStatus: DataStatus
    operations: Array<any>
}

export const RrpOperations = (props: RrpOperationsProps) => {
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
                        <h1>Operations</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};