import './Footer.css';

interface FooterProps {
}

export const Footer = (props: FooterProps) => (
    <footer>
        <div className="inner">
            <span className="tool">
                Airnodes UI Management
            </span>
            <span className="copyright">
                <a href="https://api3.org" target="_blank">API3</a> &copy; 2022
            </span>
            <span className="source">
                <a href="https://github.com/EnormousCloud/airnode">Source on Github</a>
            </span>
        </div>
    </footer>
);
