import { Footer } from '../components/Footer';
import { MenuPanel, MenuPanelProps } from './../components/MenuPanel';
import { AirnodeHeader, fromParams } from "./../components/AirnodeHeader";

interface RrpAdminsProps {
    chainId: number
    contractAddress: string
    menu: MenuPanelProps
    nodeState: any
}

export const RrpAdmins = (props: RrpAdminsProps) => {
    const { chainId, contractAddress, nodeState } = props;
    return (
        <div className="no-airnode">
            <AirnodeHeader {...fromParams(chainId, contractAddress )}/>
            <main>
                <div className="inner">
                    <MenuPanel {...props.menu} />
                    <div className="content">
                        <h1>Administators</h1>
                        {(Object.keys(nodeState.admins).length > 0) ? (
                            <pre>{JSON.stringify(nodeState.admins, null, 2)}</pre>
                        ) : (
                            <div className='block-empty'>NO DATA</div>
                        )}
                        <h2>HISTORY OF CHANGES</h2>
                        <div className='block-empty'>NO HISTORY</div>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};