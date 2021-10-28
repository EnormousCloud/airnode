import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface ChangeContractProps {
    chainId: Number
    contractAddress: String
    menu: MenuPanelProps
}

export const ChangeContract = (props: ChangeContractProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Change Contract</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};