import { Footer } from '../components/Footer';

interface ScreenSelectProps {
}

export const Select = (props: ScreenSelectProps) => {
    return (
        <div>
            <main>
                <div className="inner" style={{ minHeight: 'auto' }}>
                    <div className="content">
                        <div className="screen-empty">SELECT AIRNODE</div>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};