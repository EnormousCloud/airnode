import { Footer } from '../components/Footer';
import { Loading } from '../components/Loading';

interface LoadingScreenProps {
}

export const LoadingScreen = (props: LoadingScreenProps) => {
    return (
        <div>
            <main>
                <div className="inner" style={{ minHeight: 'auto' }}>
                    <div className="content">
                        <Loading />
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};