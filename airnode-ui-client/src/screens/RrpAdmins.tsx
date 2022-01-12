import { Footer } from '../components/Footer';
import { Loading } from '../components/Loading';
import { Operation } from '../components/Operation';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { DataStatus } from './../service/types';
import { AddressLink } from '../components/forms/EtherscanLink';

interface RrpAdminsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    nodeState: any
    dataStatus: DataStatus
    operations: any
}

const renderOps = (chainId: number, dataStatus: DataStatus, ops: any) => {
    if (dataStatus.errorMessage) return (<div>{dataStatus.errorMessage}</div>);
    if (dataStatus.isLoading) return <Loading />;
    if (!ops || ops.length === 0) return <div className='block-empty'>NO ADMINS HISTORY</div>;
    return (
        <div>
            {ops.map((op:any) => (
                <Operation key={op.tx} op={op} chainId={chainId + ''} />
            ))}
        </div>
    );
}

export const RrpAdmins = (props: RrpAdminsProps) => {
    const { chainId, contractAddress, nodeState, dataStatus, operations } = props;
    const ops = (operations) ? operations[chainId + '-' + contractAddress] || [] : [];
    const opsAdmins = ops.filter((o:any) => (o.e.type === 'SetRankAdminned' || o.e.type === 'SetRank'));
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Administrators</h1>
                        {(nodeState && nodeState.admins && Object.keys(nodeState.admins).length > 0) ? (
                            <ol className="list-admins">
                            {
                            Object.keys(nodeState.admins).map((addr: any) => (
                                <li key={addr}>
                                    <AddressLink chainId={chainId + ''} address={addr} />
                                    <span className="darken">&nbsp; rank: </span>
                                    {nodeState.admins[addr].rank}
                                </li>
                            ))
                            }</ol>
                        ) : (
                            <div className='block-empty'>NO DATA</div>
                        )}
                        <h2>HISTORY OF CHANGES</h2>
                        {renderOps(chainId, dataStatus, opsAdmins)} 
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};