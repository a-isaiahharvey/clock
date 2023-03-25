//
//  Stopwatch.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import Foundation

public class Stopwatch {
  var ptr: UnsafeMutableRawPointer

  init() {
    self.ptr = stopwatch_Stopwatch_create()
  }

  var isRunning: Bool {
    stopwatch_Stopwatch_isRunning(ptr)
  }

  var lapTimes: LapTimeVec {
    LapTimeVec(value: stopwatch_Stopwatch_lapTimes(ptr))
  }

  var elapsedTime: RDuration {
    RDuration(value: stopwatch_Stopwatch_elapsedTime(ptr))
  }

  func start() {
    stopwatch_Stopwatch_start(ptr)
  }

  func stop() {
    stopwatch_Stopwatch_stop(ptr)
  }

  func reset() {
    stopwatch_Stopwatch_reset(ptr)
  }

  func addLap() {
    stopwatch_Stopwatch_addLap(ptr)
  }

  static func formatTimeString(duration: RDuration) -> String {
    String.init(cString: stopwatch_formatTime(duration.ptr))
  }

  deinit {
    stopwatch_Stopwatch_free(ptr)
  }
}
