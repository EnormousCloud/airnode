import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface AirnodeWithdrawalsProps {
    chainId: number
    contractAddress: string
    provider: string
    xPubKey: string
    menu: MenuPanelProps
}

export const AirnodeWithdrawals = (props: AirnodeWithdrawalsProps) => {
    const { chainId, contractAddress, provider, xPubKey } = props;
    return (
        <div>
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Withdrawals</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};