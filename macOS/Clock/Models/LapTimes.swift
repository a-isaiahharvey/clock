//
//  LapTimes.swift
//  Clock
//
//  Created by Allister Harvey on 3/9/23.
//

import Foundation

class LapTime: Identifiable {
    var ptr: UnsafeMutableRawPointer;
        
    var lapNumber: UInt32 {
        stopwatch_LapTime_lapNumber(ptr)
    }
    
    var splitTime: RDuration {
        RDuration(value: stopwatch_LapTime_splitTime(ptr))
    }
    
    var totalTime: RDuration {
        RDuration(value: stopwatch_LapTime_totalTime(ptr))
    }
    
    var id: UInt32 {
        lapNumber
    }
    
    init(ptr: UnsafeMutableRawPointer!) {
        self.ptr = ptr
    }
    
    deinit {
        stopwatch_LapTime_free(ptr)
    }
}

class LapTimeVec {
    var buffer: LapTimeBuffer;
    
    var count: Int {
        buffer.len
    }
    
    init(value: LapTimeBuffer) {
        buffer = value
    }
    
    func get(index: Int) -> LapTime {
        LapTime(ptr: stopwatch_LapTime_getIndex(buffer, index))
    }
    
    func asArray() -> [LapTime] {
        var result: [LapTime] = []
        for i in 0..<self.count {
            let lapTime = self.get(index: i)
            result.append(lapTime)
        }
        return result
    }
    
    deinit {
        stopwatch_Stopwatch_freeLapTimes(buffer)
    }
}
