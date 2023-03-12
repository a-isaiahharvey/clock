//
//  LapTimeTable.swift
//  Clock
//
//  Created by Allister Harvey on 3/11/23.
//

import SwiftUI

struct LapTimeTable: View {
    var laptimes: [LapTime];
    
    init() {
        self.laptimes = []
    }
    
    init(laptimes: LapTimeVec) {
        self.laptimes = laptimes.asArray()
    }
    
    var body: some View {
        Table(laptimes.reversed()) {
            TableColumn("Lap No.", value: \LapTime.lapNumber.description)
            TableColumn("Split Time", value: \LapTime.splitTime.formatTimeString)
            TableColumn("Total Time", value: \LapTime.totalTime.formatTimeString)
        }
    }
}

struct LapTimeTable_Previews: PreviewProvider {
    static var previews: some View {
        LapTimeTable()
    }
}
