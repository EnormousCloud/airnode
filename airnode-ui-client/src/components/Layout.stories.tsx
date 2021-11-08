import { ComponentStory, ComponentMeta } from "@storybook/react";
import { AirnodeHeader, fromParams } from "./AirnodeHeader";
import { Loading } from "./Loading";
import { MenuPanel } from "./MenuPanel";
import { Footer } from "./Footer";
import { mockMenu } from "./../fixtures/menu";

const Screen = (props: any) => {
  const chainId = 4;
  const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";
  const provider = "0xac498088cd9396b3e8366d7fb267697e49235b4436971d2e3bea9955744cf3b8";
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return (
    <div>
      <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey)} />
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
}


const DataInProgress = (props: any) => {
  const chainId = 4;
  const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";
  const provider = "0xac498088cd9396b3e8366d7fb267697e49235b4436971d2e3bea9955744cf3b8";
  const xPubKey = "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return (
    <div>
      <AirnodeHeader {...fromParams(chainId, contractAddress, provider, xPubKey)} />
      <main>
        <div className="inner">
          <MenuPanel {...mockMenu} />
          <div className="content">
            <h1>OPERATIONS</h1>
            <Loading />
          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
}

export default {
  title: "Screens/Empty",
  component: Screen,
} as ComponentMeta<typeof Screen>;

const Template: ComponentStory<typeof Screen> = (args) => <Screen {...args} />;

export const Empty = Template.bind({});
Empty.args = {};

const TemplateLoading: ComponentStory<typeof Loading> = (args) => <Loading {...args} />;
export const WaitForState = TemplateLoading.bind({});

const TemplateInProgress: ComponentStory<typeof DataInProgress> = (args) => <DataInProgress {...args} />;
export const WaitForData = TemplateInProgress.bind({});
