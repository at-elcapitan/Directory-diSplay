//
//  file.swift
//  directory-display
//
//  Created by ElCapitan on 17.05.2026.
//

import Foundation

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

public struct LoadedDisplayableObjects {
    public var directories: [DisplayableObject]
    public var files: [DisplayableObject]
    public var maxNameLength: String
    public var totalSize: Int
}

// TODO: v26.1
// func getFilePermissions(...) -> ... { ... }

func loadDirectory(directoryPath: String) throws -> [URL] {
    let fileManager = FileManager.default
    let directoryURL = URL(fileURLWithPath: directoryPath)
    
    let dirContent = try fileManager.contentsOfDirectory(
        at: directoryURL,
        includingPropertiesForKeys: nil
    )
    
    return dirContent
}

/*public func loadDisplayableObjects(directoryPath: String) throws -> LoadedDisplayableObjects {
    let fileManager = FileManager.default
    let directoryURL = URL(fileURLWithPath: directoryPath)
    
    do {
    
    }
}*/
