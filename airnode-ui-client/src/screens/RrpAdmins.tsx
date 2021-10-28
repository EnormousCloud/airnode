import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface RrpAdminsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const RrpAdmins = (props: RrpAdminsProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
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