//
//  utils.swift
//  directory-display
//
//  Created by ElCapitan on 17.05.2026.
//

import Foundation

public struct Format {
    public var size: Double
    public var shortName: String
}

public func getDirName(directoryPath: String) -> String {
    let directoryURL = URL(
        fileURLWithPath: directoryPath
    )
    return directoryURL.lastPathComponent
}

public func fmt_size(size: Double) -> Format {
    let KB = 1024.0
    let MB = KB * 1024.0
    let GB = MB * 1024.0

    if size < KB {
        return Format(size: size, shortName: "B")
    } else if size < MB {
        return Format(size: size / KB, shortName: "KB")
    } else if size < GB {
        return Format(size: size / MB, shortName: "MB")
    } else {
        return Format(size: size / GB, shortName: "GB")
    }
}

public func displayHelp() {
    print("Usage: ds [OPTION] [DIRECTORY]");
    print();
    print("Display directory contents");
    print();
    print("Options:");
    print("  -a, --all             display all directory content");
    print("  -s, --size            display files size");
    print("  -f, --full-name       disable file name shortening");
    print("  -A, --access          display access options");
    print("      --version         display current program version");
    print("      --help            display this message)");
    print();
    print("Directory diSplay home page: <https://github.com/at-elcapitan/Directory-diSplay>");
}

public func displayVersion() {
    print("ds (Directory diSplay) 26.0 beta 1 for macOS");
    print();
    print(
        "This is free software; see the source for copying conditions. There is NO"
    );
    print(
        "warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE."
    );
    print();
    print("Written by Vladislav 'ElCapitan' Nazarov");
}

public func displayFirstLine(
    maxLength: Int,
    appState: ApplicationState
) {
    //var additional: String = ""
    
    // TODO: at v26.1
    // if flagSize {
    //     additional.append("   SIZE\t\t")
    // }
    
    // if flagAccess {
    //     additional.append("PUSER\tGROUP\tOTHER")
    // }
    
    let spacing = String(
        repeating: " ",
        count: maxLength + 7 - appState.directory.count
    )
    let directoryName = getDirName(directoryPath: appState.directory)
    
    print("\(directoryName)/\(spacing)") // \tTYPE\t\t\(additional)")
}

// public func displayObjects(
//    objects:
// ) {
//
// }
