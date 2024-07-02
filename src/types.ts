export interface FakeStoreProduct {
    id: number,
    title: string,
    price: number,
    category: string,
    description: string,
    image: string,
}


export interface Device {
    ISAPIParams: {
        address: string;
        addressingFormatType: string;
        portNo: number;
    };
    activeStatus: boolean;
    devIndex: string;
    devMode: string;
    devName: string;
    devStatus: string;
    devType: string;
    devVersion: string;
    protocolType: string;
    videoChannelNum: number;
}

export interface Match {
    Device: Device;
}

export interface SearchResult {
    MatchList: Match[];
    numOfMatches: number;
    totalMatches: number;
}