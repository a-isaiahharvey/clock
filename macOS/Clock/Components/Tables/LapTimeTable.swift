//
//  LapTimeTable.swift
//  Clock
//
//  Created by Allister Harvey on 3/11/23.
//

import SwiftUI

struct LapTimeTable: View {
  var laptimes: [LapTime]

  init() {
    self.laptimes = []
  }

  init(laptimes: LapTimeVec) {
    self.laptimes = laptimes.asArray()
  }

  var body: some View {
    Table(laptimes.reversed()) {
      TableColumn("Lap No.", value: \LapTime.lapNumber.description)
      TableColumn("Split Time") { lapTime in
        Text(Stopwatch.formatTimeString(duration: lapTime.splitTime))
      }
      TableColumn("Total Time") { lapTime in
        Text(Stopwatch.formatTimeString(duration: lapTime.totalTime))

      }
    }
  }
}

struct LapTimeTable_Previews: PreviewProvider {
  static var previews: some View {
    LapTimeTable()
  }
}
