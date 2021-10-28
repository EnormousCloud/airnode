import { Footer } from '../components/Footer';

interface ScreenSelectProps {
}

export const Select = (props: ScreenSelectProps) => {
    return (
        <div>
            <main>
                <div className="inner" style={{ minHeight: 'auto' }}>
                    <div className="content">
                        <h1>SELECT AIRNODE</h1>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};