import './Operation.css';
import { ExternalLink } from './forms/ExternalLink';
import { TxLink } from './forms/EtherscanLink';

interface OperationProps {
    chainId: string
    op: any
}

const niceFees = (fees: any, chainId: string): Array<string> => {
    const pieces = [];
    if (fees.gas_used) pieces.push("Gas Used " + fees.gasUsed);
    else pieces.push("Gas Limit " + fees.gas);
    pieces.push("Gas Price: " + fees.gasPrice + " GWei"); // 10-e9
    return pieces;
}

export const Operation = (props: OperationProps) => {
    const { chainId, op } = props;
    const { tm, tx, h } = op;
    const tmd = new Date(tm * 1000).toDateString();
    const tms = new Date(tm * 1000).toLocaleTimeString();
    return (
        <div className="op">
            <div className="op-row">
                <div className="op-col">
                    <div className="op-tx"><TxLink {...{ chainId, tx }} block={h} /></div>
                    <div className="op-dt darken" title={tm}>{tmd}</div>
                    <div className="op-tm darken" title={tm}>{tms}</div>
                </div>
                <div className="op-e">
                    <div className="op-type">{op.e.type}</div>
                    {Object.keys(op.e).filter(x => x != 'type').map((k:string) => {
                        return <div className="op-kv darken" key={k}>{k}: 
                            <strong className="v">{JSON.stringify(op.e[k])}</strong></div>
                    })}
                </div>
                <div className="op-fee darken">
                    {/*niceFees(op.fees, chainId).map(x => <div key={x}>{x}</div>)*/}
                </div>
            </div>
        </div>
    );
}
