import { Footer } from "./Footer";

interface ErrorScreenProps {
  error: any;
}

const niceError = (e: any): string => {
  return e + "";
};

export const ErrorScreen = (props: ErrorScreenProps) => {
  return (
    <div>
      <main>
        <div className="inner" style={{ minHeight: "auto" }}>
          <div className="content">
            <div className="warning" style={{ textAlign: "center" }}>
              {niceError(props.error)}
            </div>
          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
};
