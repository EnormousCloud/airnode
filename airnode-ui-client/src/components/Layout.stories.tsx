import Swal from "sweetalert2";
import withReactContent from "sweetalert2-react-content";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import { AirnodeHeader, fromParams } from "./AirnodeHeader";
import { ErrorScreen } from "./ErrorScreen";
import { Loading } from "./Loading";
import { MenuPanel } from "./MenuPanel";
import { Footer } from "./Footer";
import { mockMenu } from "./../fixtures/menu";

const Alerts = withReactContent(Swal);
const Toast = Swal.mixin({
  toast: true,
  position: "top-end",
  showConfirmButton: false,
  timer: 3000,
  timerProgressBar: true,
  didOpen: (toast) => {
    toast.addEventListener("mouseenter", Swal.stopTimer);
    toast.addEventListener("mouseleave", Swal.resumeTimer);
  },
});

const Screen = (props: any) => {
  const chainId = 4;
  const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";
  const provider =
    "0xac498088cd9396b3e8366d7fb267697e49235b4436971d2e3bea9955744cf3b8";
  const xPubKey =
    "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return (
    <div>
      <AirnodeHeader
        {...fromParams(chainId, contractAddress, provider, xPubKey)}
      />
      <main>
        <div className="inner">
          <MenuPanel {...mockMenu} />
          <div className="content">
            <div className="screen-empty">NO DATA</div>

            <div style={{ display: "flex", alignItems: "center", justifyContent: "space-evenly" }}>
              <div>Alerts: </div>
              <div>
                <button
                  onClick={() => {
                    Alerts.fire({
                      title: "Successs",
                      text: "You successfully hit the button",
                      icon: "success",
                    });
                  }}
                >
                  SUCCESS{" "}
                </button>
              </div>
              <div>
                <button
                  onClick={() => {
                    Alerts.fire({
                      title: "Error!",
                      text: "Do you want to continue?",
                      icon: "error",
                      confirmButtonText: "Yes",
                    });
                  }}
                >
                  FAILURE
                </button>
              </div>
            </div>
            <br />
            <div style={{ display: "flex", alignItems: "center", justifyContent: "space-evenly" }}>
              <div>Toasts: </div>
              <div>
                <button
                  onClick={() => {
                    Toast.fire({
                      title: "Successs",
                      text: "You successfully hit the button",
                      icon: "success",
                    });
                  }}
                >
                  SUCCESS{" "}
                </button>
              </div>
              <div>
                <button
                  onClick={() => {
                    Toast.fire({
                      title: "Error!",
                      text: "Failed to continue",
                      icon: "error",
                    });
                  }}
                >
                  FAILURE
                </button>
              </div>
            </div>

          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
};

const DataInProgress = (props: any) => {
  const chainId = 4;
  const contractAddress = "0xf9c39ec11055508bdda0bc2a0234abbbc09a3dec";
  const provider =
    "0xac498088cd9396b3e8366d7fb267697e49235b4436971d2e3bea9955744cf3b8";
  const xPubKey =
    "xpub661MyMwAqRbcFwK5WBQpYHeJSMehLHgHga1JEepQpYzq8t4DgFuCCbUvFLQHwtHJZHWGCL69B4XEJzctzZ8YBCorp66Q7m1UdU6YDLjfWGM";
  return (
    <div>
      <AirnodeHeader
        {...fromParams(chainId, contractAddress, provider, xPubKey)}
      />
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
};

export default {
  title: "Screens/Layout",
  component: Screen,
} as ComponentMeta<typeof Screen>;

const Template: ComponentStory<typeof Screen> = (args) => <Screen {...args} />;
export const Empty = Template.bind({});
Empty.args = {};

const TemplateLoading: ComponentStory<typeof Loading> = (args) => (
  <Loading {...args} />
);
export const WaitForState = TemplateLoading.bind({});

const TemplateInProgress: ComponentStory<typeof DataInProgress> = (args) => (
  <DataInProgress {...args} />
);
export const WaitForData = TemplateInProgress.bind({});

const TemplateError: ComponentStory<typeof DataInProgress> = (args) => (
  <ErrorScreen {...args} />
);
export const CriticalError = TemplateError.bind({});
CriticalError.args = {
  error: new Error("This is a critical error that prevents state to be loaded"),
};
