import { ComponentStory, ComponentMeta } from "@storybook/react";
import { AirnodeHeader } from "./AirnodeHeader";
import { MenuPanel, MenuPanelProps } from "./MenuPanel";
import { Footer } from "./Footer";
import { mockMenu } from "./../fixtures/menu";

const Screen = (props: any) => (
  <div>
    <AirnodeHeader filter="" />
    <main>
      <div className="inner">
        <MenuPanel {...mockMenu} />
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
