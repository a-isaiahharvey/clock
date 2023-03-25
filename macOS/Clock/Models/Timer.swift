//
//  Timer.swift
//  Clock
//
//  Created by Allister Harvey on 3/14/23.
//

import Foundation
import UserNotifications

public class RTimer: Equatable {
  public static func == (lhs: RTimer, rhs: RTimer) -> Bool {
      timer_Timer_eq(lhs.ptr, rhs.ptr)
  }
    
  var ptr: UnsafeMutableRawPointer

  init(secs: UInt64) {
    self.ptr = timer_Timer_create(secs)
  }

  var isRunning: Bool {
    timer_Timer_isRunning(ptr)
  }

  var isDone: Bool {
    timer_Timer_isDone(ptr)
  }

  var hasNotStarted: Bool {
    timer_Timer_hasNotStarted(ptr)
  }

  var elapsed: RDuration {
    RDuration(value: timer_Timer_elapsed(ptr))
  }

  var duration: RDuration {
    RDuration(value: timer_Timer_duration(ptr))
  }

  func start() {
    timer_Timer_start(ptr)
  }

  func stop() {
    timer_Timer_stop(ptr)
  }

  func reset() {
    timer_Timer_reset(ptr)
  }

  func remaining() -> RDuration {
    RDuration(value: timer_Timer_remaining(ptr))
  }

  deinit {
    timer_Timer_free(ptr)
  }
}
