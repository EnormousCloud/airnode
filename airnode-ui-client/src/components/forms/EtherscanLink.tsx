import { ExternalLink } from "./ExternalLink";
import { getChainEtherscan } from './../../service/Chain';

export const svgExternal = (size: number) => (
  <svg
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    className="link-icon"
    style={{ width: size, height: size }}
    viewBox="0 0 511.626 511.627"
  >
    <g>
      <path
        d="M392.857,292.354h-18.274c-2.669,0-4.859,0.855-6.563,2.573c-1.718,1.708-2.573,3.897-2.573,6.563v91.361
            c0,12.563-4.47,23.315-13.415,32.262c-8.945,8.945-19.701,13.414-32.264,13.414H82.224c-12.562,0-23.317-4.469-32.264-13.414
            c-8.945-8.946-13.417-19.698-13.417-32.262V155.31c0-12.562,4.471-23.313,13.417-32.259c8.947-8.947,19.702-13.418,32.264-13.418
            h200.994c2.669,0,4.859-0.859,6.57-2.57c1.711-1.713,2.566-3.9,2.566-6.567V82.221c0-2.662-0.855-4.853-2.566-6.563
            c-1.711-1.713-3.901-2.568-6.57-2.568H82.224c-22.648,0-42.016,8.042-58.102,24.125C8.042,113.297,0,132.665,0,155.313v237.542
            c0,22.647,8.042,42.018,24.123,58.095c16.086,16.084,35.454,24.13,58.102,24.13h237.543c22.647,0,42.017-8.046,58.101-24.13
            c16.085-16.077,24.127-35.447,24.127-58.095v-91.358c0-2.669-0.856-4.859-2.574-6.57
            C397.709,293.209,395.519,292.354,392.857,292.354z"
      ></path>
      <path
        d="M506.199,41.971c-3.617-3.617-7.905-5.424-12.85-5.424H347.171c-4.948,0-9.233,1.807-12.847,5.424
            c-3.617,3.615-5.428,7.898-5.428,12.847s1.811,9.233,5.428,12.85l50.247,50.248L198.424,304.067
            c-1.906,1.903-2.856,4.093-2.856,6.563c0,2.479,0.953,4.668,2.856,6.571l32.548,32.544c1.903,1.903,4.093,2.852,6.567,2.852
            s4.665-0.948,6.567-2.852l186.148-186.148l50.251,50.248c3.614,3.617,7.898,5.426,12.847,5.426s9.233-1.809,12.851-5.426
            c3.617-3.616,5.424-7.898,5.424-12.847V54.818C511.626,49.866,509.813,45.586,506.199,41.971z"
      ></path>
    </g>
  </svg>
);

export interface AddressLinkProps {
  chainId: string
  address: string
}

const shortened = (x: string) => {
    if (!x.startsWith('0x') || x.length < 10) return x;
    return x.substring(0, 6) + '...' + x.substring(x.length-4);
}

export const AddressLink = (props: AddressLinkProps) => {
  const baseUrl = getChainEtherscan(props.chainId);
  if (!baseUrl) return null;
  const href = baseUrl + "/address/" + props.address;
  return (
    <ExternalLink href={href} className="icon" title="View on Etherscan">
      {shortened(props.address)} &nbsp; {svgExternal(16)}
    </ExternalLink>
  );
};

export const AddressIcon = (props: AddressLinkProps) => {
  const baseUrl = getChainEtherscan(props.chainId);
  if (!baseUrl) return null;
  const href = baseUrl + "/address/" + props.address;
  return (
    <ExternalLink href={href} className="icon" title="View on Etherscan" style={{ paddingLeft: 5, paddingRight: 5 }}>
      {svgExternal(12)}
    </ExternalLink>
  );
};

export interface TxLinkProps {
    chainId: string
    block: number
    tx: string;
}

const numberWithCommas = (x: number) => {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

export const TxLink = (props: TxLinkProps) => {
    const baseUrl = getChainEtherscan(props.chainId);
    if (!baseUrl) return null;
    const href = baseUrl + "/tx/" + props.tx;
    return (
      <ExternalLink href={href} title="View on Etherscan">
        {numberWithCommas(props.block)}
      </ExternalLink>
    );
  };
  