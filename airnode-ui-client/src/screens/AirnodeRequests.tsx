import { Footer } from '../components/Footer';
import { Link } from 'react-router-dom';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeRequestsProps {
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

export const AirnodeRequests = (props: AirnodeRequestsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey, requests } = airnodeState;
    const opsURL = "/" + chainId + "/" + contractAddress + '/nodes/' + provider + '/operations';
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Requests</h1>
                        {(Object.keys(requests).length > 0) ? (
                            <ol>
                                {Object.keys(requests).map((rq: string) => (
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
                            <div className='block-empty'>NO ENDPOINTS</div> 
                        )}
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};