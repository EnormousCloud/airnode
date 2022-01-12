import './Footer.css';
import { ExternalLink } from './forms/ExternalLink';
interface FooterProps {
}

export const Footer = (props: FooterProps) => (
    <footer>
        <div className="inner">
            <span className="tool">
                Airnodes UI Management
            </span>
            <span className="copyright">
                <ExternalLink href="https://api3.org">API3 DAO</ExternalLink> &copy; 2022
            </span>
            <span className="source">
                <a href="https://github.com/EnormousCloud/airnode">Source on Github</a>
            </span>
        </div>
    </footer>
);
