// basic data loading status
export interface DataStatus {
    // whether data is loading
    isLoading: boolean
    // error message if data failed to load
    errorMessage: string
}

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
