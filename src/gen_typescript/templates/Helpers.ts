export type EventListener = (breezEvent: BreezEvent) => void

export type LogStream = (logEntry: LogEntry) => void

export const connect = async (config: Config, seed: number[], listener: EventListener): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkEvent", listener)
    
    await BreezSDK.connect(config, seed)

    return subscription
}

export const setLogStream = async (logStream: LogStream): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkLog", logStream)

    await BreezSDK.setLogStream()

    return subscription
}
