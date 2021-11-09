import { Footer } from "./Footer";
import { AirnodeHeader, noParams } from "./AirnodeHeader";

interface ErrorScreenProps {
  error: any;
}

const niceError = (e: any): string => {
  return e + "";
};

export const ErrorScreen = (props: ErrorScreenProps) => {
  return (
    <div className="no-airnodes">
      <AirnodeHeader {...noParams()} />
      <main>
        <div className="inner" style={{ minHeight: "auto" }}>
          <div className="content">
            <div className="warning" style={{ border: '1px var(--color-error) solid', textAlign: "center", paddingTop: 50, paddingBottom: 50  }}>
              {niceError(props.error)}
            </div>
          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
};
