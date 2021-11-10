import { ComponentStory, ComponentMeta } from '@storybook/react';

import { AirnodeHeader } from './AirnodeHeader';

export default {
  title: 'Components/Header',
  component: AirnodeHeader,
} as ComponentMeta<typeof AirnodeHeader>;

const Template: ComponentStory<typeof AirnodeHeader> = (args) => <AirnodeHeader {...args} />;

export const Default = Template.bind({});
Default.args = {
  filter: "",
  chain: 'RINKEBY',
  contract: '0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec',
  airnode: '0x19255a4ec31e89cea54d1f125db7536e874ab4a96b4d4f6438668b6bb10a6adb',
  hrefSelect: '/',
  xpubkey: "xpub661MyMwAqRbcGeCE1g3KTUVGZsFDE3jMNinRPGCQGQsAp1nwinB9Pi16ihKPJw7qtaaTFuBHbRPeSc6w3AcMjxiHkAPfyp1hqQRbthv4Ryx",
};


export const NoAirnode = Template.bind({});
NoAirnode.args = {
  filter: "",
  chain: 'RINKEBY',
  contract: '0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec',
  airnode: '',
  hrefSelect: '/',
  xpubkey: "",
};

export const Filtered = Template.bind({});
Filtered.args = {
  filter: "/",
  filterName: "Endpoint ID: 0xf466...453c",
  chain: 'RINKEBY',
  contract: '0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec',
  airnode: '0x19255a4ec31e89cea54d1f125db7536e874ab4a96b4d4f6438668b6bb10a6adb',
  hrefSelect: '/',
  xpubkey: "",
};
