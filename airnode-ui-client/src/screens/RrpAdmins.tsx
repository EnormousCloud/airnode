import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface RrpAdminsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const RrpAdmins = (props: RrpAdminsProps) => {
    const { chainId, contractAddress } = props;
    const provider = '';
    const xPubKey = '';
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Administators</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};