import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface AirnodeEndpointsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
}

export const AirnodeEndpoints = (props: AirnodeEndpointsProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Airnode Endpoints</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};