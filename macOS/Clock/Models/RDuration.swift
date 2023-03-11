//
//  RDuration.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import Foundation

public class RDuration {
    var ptr: UnsafeMutableRawPointer;
    
    init(value: UnsafeMutableRawPointer!) {
        ptr = value
    }
    
    var formatTimeString: String {
        String.init(cString: stopwatch_formatTime(ptr))
    }
    
    deinit {
        stopwatch_freeDuration(ptr)
    }
}
