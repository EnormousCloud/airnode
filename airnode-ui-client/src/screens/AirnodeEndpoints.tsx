import { Footer } from '../components/Footer';
import { Link } from 'react-router-dom';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeEndpointsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
    airnodeState: any
}

const shortened = (x: string, ln: number) => {
    if (!x || !x.startsWith("0x") || x.length <= ln * 2 + 2) return x;
    return x.substring(0, ln + 2) + "..." + x.substring(x.length - ln);
};

export const AirnodeEndpoints = (props: AirnodeEndpointsProps) => {
    const { chainId, contractAddress, provider, airnodeState } = props;
    const { xpubkey, endpoints } = airnodeState;
    const opsURL = "/" + chainId + "/" + contractAddress + '/nodes/' + provider + '/operations';
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider)} xpubkey={xpubkey} />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Endpoints</h1>
                        {(Object.keys(endpoints).length > 0) ? (
                            <ol>
                                {Object.keys(endpoints).map((ep: string) => (
                                    <li key={ep}>
                                        <Link style={{ fontFamily: 'monospace' }} to={ opsURL + '/endpoint/' + ep}>
                                            {shortened(ep, 12)}
                                        </Link>
                                        &nbsp;
                                        <small className="darken">{endpoints[ep]} request{endpoints[ep] > 1 ? 's':''}</small>
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