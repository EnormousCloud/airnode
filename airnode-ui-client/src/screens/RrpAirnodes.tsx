import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface RrpAirnodesProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const RrpAirnodes = (props: RrpAirnodesProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Airnodes</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};