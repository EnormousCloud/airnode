import { ComponentStory, ComponentMeta } from '@storybook/react';

import { AirnodeHeader } from './AirnodeHeader';
import { Menu, MenuItem } from './Menu';
import { MenuPanel } from './MenuPanel';
import { Footer } from './Footer';
import { Balance } from './Balance';

const itemsAirnode: Array<MenuItem> = [
    {
      name: "Operations",
      counter: 100,
      href: "/",
    },
    {
      name: "Endpoints",
      counter: 3,
      href: "/",
    },
    {
      name: "Whitelists",
      counter: 0,
      href: "/",
    },
    {
      name: "Withdrawals",
      href: "/",
    },
];


const itemsRRP: Array<MenuItem> = [
    {
      name: "Operations",
      counter: 450,
      href: "/",
    },
    {
      name: "Admins",
      counter: 2,
      href: "/",
    }
];

const Screen = (props: any) => (
    <div>
        <AirnodeHeader filter="" />
        <main>
            <div className="inner">
                <MenuPanel>
                    <h3>Airnode</h3>
                    <div className="menu-address">
                        <a href="#">0xeeee...0000</a>
                    </div>
                    <Menu items={itemsAirnode} />
                    <Balance value='0.393993' symbol='ETH' />

                    <h3>RRP contract</h3>
                    <div className="menu-address">
                        <a href="#">0x4444...0000</a>
                    </div>
                    <Menu items={itemsRRP} />
                    <Balance value='0.000000' symbol='ETH' />
                </MenuPanel>
                <div className="content">
                    <div className="screen-empty">NO ENDPOINTS</div>
                </div>
            </div>
        </main>
        <Footer />
    </div>
);

export default {
    title: "Screens/Empty",
    component: Screen,
  } as ComponentMeta<typeof Screen>;

const Template: ComponentStory<typeof Screen> = (args) => <Screen {...args} />;
  
export const Empty = Template.bind({});
Empty.args = {};
