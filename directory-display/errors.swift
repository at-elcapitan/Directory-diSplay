//
//  errors.swift
//  directory-display
//
//  Created by ElCapitan on 18.05.2026.
//

enum AppStateErrors: Error {
    case UnknownArgument(String)
}

enum DisplayableObjectsLoadingError: Error {
    case UnableToReadDirectory
}
