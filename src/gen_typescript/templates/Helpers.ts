export type EventListener = (breezEvent: BreezEvent) => void

export type LogStream = (logEntry: LogEntry) => void

export type Logger = (logMessage: LogMessage) => void

export const connect = async (config: Config, seed: number[], listener: EventListener, nodeLogger: Logger, logFilePath?: string ): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkEvent", listener)
    
    BreezSDKEmitter.addListener("breezSdkNodeLog", nodeLogger)

    await BreezSDK.connect(config, seed, logFilePath)

    return subscription
}

export const setLogStream = async (logStream: LogStream): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkLog", logStream)

    await BreezSDK.setLogStream()

    return subscription
}
