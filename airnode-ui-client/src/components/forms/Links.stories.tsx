import { ComponentStory, ComponentMeta } from '@storybook/react';
import { AddressLink, TxLink } from './EtherscanLink';

export default {
  title: 'Components/Links',
  component: AddressLink,
} as ComponentMeta<typeof AddressLink>;

const TemplateAddress: ComponentStory<typeof AddressLink> = (args) => (
    <div className="centered">
        <div style={{ padding: 20, textAlign: 'center' }}><AddressLink {...args} /></div>
    </div>
);

export const Address = TemplateAddress.bind({});
Address.args = {
    chainId: "0",
    address: '0x6518c695cdcbefa272a4e5ef73bd46e801983e19'
};

export const Rinkeby = TemplateAddress.bind({});
Rinkeby.args = {
    chainId: "4",
    address: '0x6518c695cdcbefa272a4e5ef73bd46e801983e19'
};

const TemplateTx: ComponentStory<typeof TxLink> = (args) => (
    <div className="centered">
        <div style={{ padding: 20, textAlign: 'center' }}><TxLink {...args} /></div>
    </div>
);

export const Transaction = TemplateTx.bind({});
Transaction.args = {
    chainId: "0",
    tx: '0xced5417f76581dc009fa277d924a8dce5e50e01d8c11792992a8a473e6bad69b',
    block: 13506437,
};