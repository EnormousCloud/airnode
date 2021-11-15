// import './Operation.css';

import { ExternalLink } from './forms/ExternalLink';
import { AddressLink } from './forms/EtherscanLink';

interface OperationProps {
}

export const Operation = (props: OperationProps) => (
    <div className="op">
        <pre>{JSON.stringify(props, null, 2)}</pre>
    </div>
);
