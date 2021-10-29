import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface AirnodeOperationsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
}

export const AirnodeOperations = (props: AirnodeOperationsProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Operations</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};