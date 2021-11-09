// basic data loading status
export interface DataStatus {
    // whether data is loading
    isLoading: boolean
    // error message if data failed to load
    errorMessage: string
}

export const DataIsLoading: DataStatus = {
    isLoading: true,
    errorMessage: '',
};

export const DataIsReady: DataStatus = {
    isLoading: false,
    errorMessage: '',
};

// additional information about paging
export interface More {
    // token from which the request should start
    from?: string
    // optional total amount of items that will follow
    total?: number
}

// extended data status - for "load more" pager
export interface InfinityDataStatus {
    // whether initial screen is loading
    isLoading: boolean
    // global error message to block screen on initial load
    errorMessage: string
    // whether there is more to load
    canLoadMore: boolean
    // whether initial screen is loading
    isLoadingMore: boolean
    // key of the last element to load from
    more?: More
    // error message on loading more
    errorLoadingMore: string
}

// cursor for load-more paginated requests
export interface Cursor {
    // limit number of results
    limit?: number
    // limit
    from?: string
}

// Airnode context selection
export interface AirnodeRef {
    // EVM chain ID
    chainId: number
    // Address of the RRP contract
    contractAddress: string
    // Provider ID (for pre-alpha airnode). None might be selected
    provider?: string
}

// Airnode context selection
export interface AirnodeFullRef {
    // EVM chain ID
    chainId: number
    // Address of the RRP contract
    contractAddress: string
    // Provider ID (for pre-alpha airnode). None might be selected
    provider?: string
    // Address of the Airnode (v0.2 and later). None might be selected
    airnode?: string
    // Restrict view by the only endpoint
    by_endpoint?: string
    // Restrict view by the only endpoint
    by_template?: string
    // Restrict view by the only endpoint
    by_func?: string
}

// State that is persistent in session storage
export interface PersistentState {
    filters: Array<AirnodeRef>
}