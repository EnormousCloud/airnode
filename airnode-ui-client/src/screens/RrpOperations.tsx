import { Footer } from '../components/Footer';
import { Loading } from './../components/Loading';
import { Operation } from './../components/Operation';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { DataStatus } from './../service/types';

interface RrpOperationsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    dataStatus: DataStatus
    operations: any
}

export const RrpOperations = (props: RrpOperationsProps) => {
    const { chainId, contractAddress, dataStatus, operations } = props;
    const ops = (operations) ? operations[chainId + '-' + contractAddress] || [] : [];
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                        {(dataStatus.errorMessage) ? (
                            <div>{dataStatus.errorMessage}</div>
                        ): ((dataStatus.isLoading) ? (
                                <Loading />
                            ): ops.map((op:any) => (
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