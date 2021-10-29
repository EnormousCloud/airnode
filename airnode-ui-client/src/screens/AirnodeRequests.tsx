import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface AirnodeRequestsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
}

export const AirnodeRequests = (props: AirnodeRequestsProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Airnode Requests</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};