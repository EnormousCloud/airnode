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
    filterName?: string
    filterValue?: string
    menu: MenuPanelProps
    dataStatus: DataStatus
    airnodeState: any
    operations: any
}

const renderOps = (chainId: number, dataStatus: DataStatus, ops: any) => {
    if (dataStatus.errorMessage) return (<div>{dataStatus.errorMessage}</div>);
    if (dataStatus.isLoading) return <Loading />;
    if (!ops || ops.length === 0) return <div className='block-empty'>NO HISTORY</div>;
    return (
        <div>
            {ops.map((op:any) => (
                <Operation key={op.tx} op={op} chainId={chainId + ''} />
            ))}
        </div>
    );
}

export const AirnodeOperations = (props: AirnodeOperationsProps) => {
    const { chainId, contractAddress, provider, airnodeState, dataStatus, operations } = props;
    const { filterName, filterValue } = props;
    const { xpubkey } = airnodeState;
    const opsAll = (operations) ? operations[chainId + '-' + contractAddress] || [] : [];
    const ops = opsAll.filter((op: any) => {
        if (!op.e) return false
        if (op.e.provider_id !== provider && op.e.airnode !== provider) return false;
        if (filterName && filterValue) {
            if (filterName == 'request') return (op.e.request_id == filterValue);
            if (filterName == 'endpoint') return (op.e.endpoint_id == filterValue);
            if (filterName == 'from') return (op.from == filterValue);
            if (filterName == 'tx') return (op.tx == filterValue);
        }
        return true;
    });
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                        {(filterName && filterValue) ? (
                            <div className="filtered darken">
                                Filtered by {filterName}: <strong>{filterValue}</strong>
                            </div>
                        ): null}
                        {renderOps(chainId, dataStatus, ops)}
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};