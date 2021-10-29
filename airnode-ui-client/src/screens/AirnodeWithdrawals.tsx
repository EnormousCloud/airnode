import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader } from "./../components/AirnodeHeader";

interface AirnodeWithdrawalsProps {
    chainId: number
    contractAddress: string
    provider: string
    menu: MenuPanelProps
}

export const AirnodeWithdrawals = (props: AirnodeWithdrawalsProps) => {
    return (
        <div>
            <AirnodeHeader filter="" />
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