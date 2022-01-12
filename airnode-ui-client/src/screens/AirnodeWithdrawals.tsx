import { Link } from 'react-router-dom';
import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeWithdrawalsProps {
    chainId: number
    contractAddress: string
    provider: string
    airnodeState: any
    menu: MenuPanelProps
}

const shortened = (x: string, ln: number) => {
    if (!x || !x.startsWith("0x") || x.length <= ln * 2 + 2) return x;
    return x.substring(0, ln + 2) + "..." + x.substring(x.length - ln);
};

export const AirnodeWithdrawals = (props: AirnodeWithdrawalsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey, requests } = airnodeState;
    const opsURL = "/" + chainId + "/" + contractAddress + '/nodes/' + provider + '/operations';
    const filtered = Object.keys(requests).filter((rq: string) => (
        !!requests[rq].withdraw
    ));
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Withdrawals</h1>
                        {(filtered.length > 0) ? (
                            <ol>
                                {filtered.map((rq: string) => (
                                    <li key={rq}>
                                        <Link style={{ fontFamily: 'monospace' }} to={ opsURL + '/request/' + rq}>
                                            {shortened(rq, 16)}
                                        </Link>
                                        &nbsp;
                                        {(requests[rq].fulfill) ? (
                                            <span className='badge badge-ok'>Fulfilled</span>
                                        ): null}
                                        {(requests[rq].fail) ? (
                                            <span className='badge badge-error'>Fail</span>
                                        ): null}
                                    </li>
                                ))}
                            </ol>
                        ): (
                            <div className='block-empty'>NO WITHDRAWALS</div> 
                        )}
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};