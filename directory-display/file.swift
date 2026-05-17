//
//  file.swift
//  directory-display
//
//  Created by ElCapitan on 17.05.2026.
//

let MAX_FILE_NAME_LEN = 38

public struct ObjectType {
    public var typeName: String
    public var displayColor: String
}

public struct DisplayableObject {
    public var size: UInt
    public var objectName: String
    public var objectType: ObjectType
    // TODO: v26.1
    // public var permissions: Permissions
}

// TODO: v26.1
// func getFilePermissions(...) -> ... { ... }

