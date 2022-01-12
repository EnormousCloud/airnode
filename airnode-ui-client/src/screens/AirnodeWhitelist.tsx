import { Footer } from '../components/Footer';
import { Loading } from './../components/Loading';
import { Operation } from './../components/Operation';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";
import { DataStatus } from './../service/types';

interface AirnodeWhitelistProps {
    chainId: number
    contractAddress: string
    provider: string
    dataStatus: DataStatus
    airnodeState: any
    operations: any
    menu: MenuPanelProps
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

export const AirnodeWhitelist = (props: AirnodeWhitelistProps) => {
    const { chainId, contractAddress, provider, airnodeState, dataStatus, operations } = props;
    const { xpubkey, whitelisted } = airnodeState;
    const opsAll = (operations) ? operations[chainId + '-' + contractAddress] || [] : [];
    const ops = opsAll.filter((op: any) => {
        if (!op.e) return false
        if (op.e.provider_id !== provider && op.e.airnode !== provider) return false;
        return ([""].indexOf(op.e.type) !== -1);
    });
    
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Whitelisted Wallets</h1>
                        {(whitelisted.length > 0) ? (
                            <ol>
                                {whitelisted.map((wallet:any) => (
                                    <li>
                                        <span className="cell-title">{wallet}</span>
                                        &nbsp;
                                        <span className="badge badge-ok">Active</span>
                                    </li>
                                ))}
                            </ol>
                        ): (
                            <div className='block-empty'>NO WALLETS</div>
                        )}
                        <h1>Whitelist History</h1>
                        {renderOps(chainId, dataStatus, ops)} 
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};