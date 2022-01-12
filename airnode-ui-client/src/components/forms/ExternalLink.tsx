import styled from "styled-components";

const IE11Component = styled.a`
  margin-left: 0.33em;
  margin-right: 0.33em;
`;

const ALink = styled(IE11Component)`
  @supports (display: grid) {
    margin-inline-start: 0.33em;
    margin-inline-end: 0.33em;
  }
`;

export const ExternalLink = (props: any) => {
  const inner = props.children || props.href;
  const title = props.title;
  const { href, className, style } = props;
  return (
    <ALink
      {...{ title, href, className, style }}
      rel="noopener noreferrer nofollow"
      target="_blank"
    >
      {inner}
    </ALink>
  );
};
