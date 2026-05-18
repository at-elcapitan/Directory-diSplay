//
//  main.swift
//  directory-display
//
//  Created by ElCapitan on 4/19/2026.
//

import Foundation

func main() {
    let args = Array(CommandLine.arguments.dropFirst())
    var appState: ApplicationState?
        
    // Loading application state (arguments)
    do {
        appState = try parseArguments(args: args)
    } catch AppStateErrors.UnknownArgument(let errText) {
        print(errText)
        return
    } catch {
        print("Unexpected error: ", error)
        return
    }
    
    guard let appStateLoaded = appState else {
        print("Unable to load application state")
        return
    }

    if appStateLoaded.help {
        displayHelp()
        return
    }
    
    if appStateLoaded.version {
        displayVersion()
        return
    }
    
    var directoryFiles: [URL]?
    
    do {
        directoryFiles = try loadDirectory(directoryPath: appStateLoaded.directory)
    } catch let error as CocoaError {
        switch error.code {
        case .fileReadNoSuchFile:
            print("Error: The directory path does not exist.")
            
        case .fileReadNoPermission:
            print("Error: Permission denied. Access to this folder is restricted.")
            
        default:
            print("System error reading directory: \(error.localizedDescription)")
        }
        
        print(appState!)
        return
    } catch {
        print("An unexpected error occurred: \(error)")
        return
    }
    
    guard let directoryFilesLoaded = directoryFiles else {
        print("Directory files could not be loaded due to unexpected app behavior")
        return
    }
    
    // Calculating final max length
    let maxLength = appStateLoaded.directory.count
    
    displayFirstLine(
        maxLength: maxLength,
        appState: appStateLoaded
    )
    
    print("  │")
    
    for (index, dir) in directoryFilesLoaded.enumerated() {

        let isLast = index == directoryFilesLoaded.count - 1
        let prefix = isLast ? "└" : "├"

        print("  \(prefix) \(dir.lastPathComponent)")
    }
}

main()
