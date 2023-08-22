export type EventFn = (breezEvent: BreezEvent) => void

export type LogEntryFn = (logEntry: LogEntry) => void

export const addEventListener = (eventFn: EventFn): EmitterSubscription => {
    return BreezSDKEmitter.addListener("breezSdkEvent", eventFn)
}

export const addLogListener = async (logEntryFn: LogEntryFn): Promise<EmitterSubscription> => {
    const subscription = BreezSDKEmitter.addListener("breezSdkLog", logEntryFn)

    await BreezSDK.startLogStream()

    return subscription
}
