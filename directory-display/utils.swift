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
