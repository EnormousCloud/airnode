const root = () => {
    return import.meta.env.VITE_API_URL ? 
        import.meta.env.VITE_API_URL + '' : 
        (import.meta.env.BASE_URL || './');
};

export const API = {
    fetchState: () => fetch(root() + 'api/states').then(r => r.json()),
    fetchOps: (chainId: number, address: string) => fetch(root() + 'api/operations/' + chainId + "/" + address).then(r => r.json())
};
