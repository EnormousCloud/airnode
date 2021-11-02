import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface ChangeFilterProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const ChangeFilter = (props: ChangeFilterProps) => {
    const { chainId, contractAddress } = props;
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Change Filter</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};