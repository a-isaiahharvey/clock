//
//  Stopwatch.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import Foundation

public class Stopwatch {
    var ptr = stopwatch_stopwatch_create()
    
    var isRunning: Bool {
        stopwatch_stopwatch_isRunning(ptr)
    }
    
    var lapTimes: LapTimeVec {
        LapTimeVec(value: stopwatch_stopwatch_lapTimes(ptr))
    }
    
    var elapsedTime: RDuration {
        RDuration(value: stopwatch_stopwatch_elapsedTime(ptr))
    }
    
    func start() {
        stopwatch_stopwatch_start(ptr)
    }
    
    func stop() {
        stopwatch_stopwatch_stop(ptr)
    }
    
    func reset() {
        stopwatch_stopwatch_reset(ptr)
    }
    
    func addLap() {
        stopwatch_stopwatch_addLap(ptr)
    }
    
    deinit {
        stopwatch_stopwatch_free(ptr)
    }
}

