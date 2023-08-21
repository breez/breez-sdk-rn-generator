
static func validateMandatoryFields(map: [String: Any?], keys: [String]) throws -> Void {
    for k in keys {
        if (map[k] == nil) {
            throw SdkError.Generic(message: "Missing mandatory field")
        }
    }
}