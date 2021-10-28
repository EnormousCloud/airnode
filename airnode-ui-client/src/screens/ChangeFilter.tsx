import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface ChangeFilterProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
}

export const ChangeFilter = (props: ChangeFilterProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
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