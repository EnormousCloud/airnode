import { Footer } from '../components/Footer';

interface AddContractProps {
}

export const AddContract = (props: AddContractProps) => {
    return (
        <div>
            <main>
                <div className="inner" style={{ minHeight: 'auto' }}>
                    <div className="content">
                        <div className="screen-empty">ADD RRP CONTRACT</div>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};