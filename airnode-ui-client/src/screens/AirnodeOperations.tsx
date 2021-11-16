import { Footer } from '../components/Footer';
import { Loading } from './../components/Loading';
import { Operation } from './../components/Operation';
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
    operations: any
}

export const AirnodeOperations = (props: AirnodeOperationsProps) => {
    const { chainId, contractAddress, provider, airnodeState, dataStatus, operations } = props;
    const { xpubkey } = airnodeState;
    const ops = (operations) ? operations[chainId + '-' + contractAddress] || [] : [];
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                        {(dataStatus.errorMessage) ? (
                            <div>{dataStatus.errorMessage}</div>
                        ): ((dataStatus.isLoading) ? (
                                <Loading />
                            ): ops.filter((op: any) => (
                                op.e && op.e.provider_id === provider || op.e.airnode === provider
                            )).map((op:any) => (
                                <Operation key={op.tx} op={op} chainId={chainId + ''} />
                            ))
                        )}                        
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};