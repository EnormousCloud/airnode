import { ComponentStory, ComponentMeta } from '@storybook/react';

import { Balance } from './Balance';

export default {
  title: 'Components/Balance',
  component: Balance,
} as ComponentMeta<typeof Balance>;

const Template: ComponentStory<typeof Balance> = (args) => <Balance {...args} />;

export const ETH = Template.bind({});
ETH.args = {
    value: '0.127839',
    symbol: 'ETH'
};

export const USD = Template.bind({});
USD.args = {
    value: '0.33',
    symbol: 'USD'
};

export const NoSymbol = Template.bind({});
NoSymbol.args = {
    value: '999.99'
};
