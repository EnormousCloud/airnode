import styled from 'styled-components';

interface BalanceProps {
    value: string
    symbol?: string
}

const Div = styled.div`
    text-align: right;
    padding-right: 20px;
    .value {
        font-weight: normal;
    }
    .symbol {
        color: var(--color-panel-border);
    }
`;

export const Balance = (props: BalanceProps) => (
    <Div className="balance cell-t">
        <div className="cell-title">Balance:</div>
        <span className="value">{props.value}</span>
        {(props.symbol) ? (
            <span className="symbol">&nbsp; {props.symbol}</span>
        ): null}        
    </Div>
);