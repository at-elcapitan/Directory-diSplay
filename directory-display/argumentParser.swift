//
//  argumentParser.swift
//  directory-display
//
//  Created by ElCapitan on 17.05.2026.
//

import Foundation

public struct ApplicationState {
    public var help: Bool
    public var version: Bool
    public var access: Bool
    public var size: Bool
    public var fullName: Bool
    public var all: Bool
    public var directory: String
}

public func parseArguments(args: Array<String>) throws -> ApplicationState {
    var applicationState = ApplicationState(
        help: false,
        version: false,
        access: false,
        size: false,
        fullName: false,
        all: false,
        directory: "./"
    )
    
    for arg in args {
        switch arg {
        case "--help":
            applicationState.help = true
            
        case "--version":
            applicationState.version = true
            
        case "--access":
            applicationState.access = true
            
        case "--size":
            applicationState.size = true
            
        case "--all":
            applicationState.all = true
            
        case "--full":
            applicationState.fullName = true
            
        default:
            if !arg.starts(with: "-") {
                applicationState.directory = arg
                continue
            }
            
            let unixFlags = arg.dropFirst()
            
            for c in unixFlags {
                switch c {
                case "h":
                    applicationState.help = true
                    
                case "A":
                    applicationState.access = true
                    
                case "a":
                    applicationState.all = true
                    
                case "s":
                    applicationState.size = true
                    
                case "f":
                    applicationState.fullName = true
                
                default:
                    throw AppStateErrors.UnknownArgument("Unknown parameter \(arg)")
                }
            }
        }
    }
    
    return applicationState
}
