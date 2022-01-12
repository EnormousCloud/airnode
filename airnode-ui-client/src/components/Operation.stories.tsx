import { ComponentStory, ComponentMeta } from "@storybook/react";
import { Operation as Op } from './Operation';

export default {
  title: "Airnode/Operation",
  component: Op,
} as ComponentMeta<typeof Op>;

const Template: ComponentStory<typeof Op> = (args) => (
  <div className="inner"><div className="content"><Op {...args} /><Op {...args} /></div></div>
);

export const Operation = Template.bind({});
Operation.args = {
    chainId: "4",
    op: {
        "tm": 1620216405,
        "e": {
        "type": "EndpointUpdatedA",
        "provider_id": "0x615f0b0253e25610f82407dde4b65f7e202b366e74696888106e0bfd42f41242",
        "endpoint_id": "0xf466b8feec41e9e50815e0c9dca4db1ff959637e564bb13fefa99e9f9f90453c",
            "authorizers": [
                "0x0000000000000000000000000000000000000000"
            ]
        },
        "h": 1819151,
        "from": "0x1b8671865da057b1603c528c654bf5bc799f442f",
        "b": "0xe2ceae6f3c44c79336e65caf3a8ce7ceb6372ae0e58b346a4a1db2be622e3f8e",
        "tx": "0x31a1bc098273969eaf6b18cf51c3c1aa839b113686de47cb2ad06ece32474fbc",
        "ti": 0,
        "li": 0,
        "fees": {
            "gasPrice": "0x1a13b8600",
            "gas": "0x30d40",
            "gasUsed": "0xd880",
            "usd": 1.01
        }
    }
};