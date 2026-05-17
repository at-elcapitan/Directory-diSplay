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
    
    do {
        appState = try parseArguments(args: args)
    } catch AppStateErrors.UnknownArgument(let errText) {
        print(errText)
        return
    } catch {
        print("Unexpected error: ", error)
        return
    }
    
    if appState?.help == true {
        displayHelp()
        return
    }
    
    if appState?.version == true {
        displayVersion()
        return
    }
}

main()
