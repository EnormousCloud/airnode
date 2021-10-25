import { ComponentStory, ComponentMeta } from '@storybook/react';

import { AirnodeHeader } from './AirnodeHeader';
import { Menu, MenuItem } from './Menu';
import { MenuPanel } from './MenuPanel';
import { Footer } from './Footer';

const items: Array<MenuItem> = [
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

const Screen = (props: any) => (
    <div>
        <AirnodeHeader filter="" />
        <main>
            <div className="inner">
                <MenuPanel><Menu items={items} /></MenuPanel>
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
