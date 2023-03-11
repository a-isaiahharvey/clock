//
//  StopwatchView.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import SwiftUI

struct StopwatchView: View {
    @State private var stopwatch: Stopwatch = Stopwatch()
    @State private var elapsedTime = "00:00.00"
    @State private var timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    
    var body: some View {
        VStack {
            Text(elapsedTime).font(.system(size: 100)).fontWeight(.light).monospacedDigit().onAppear {
                let queue = DispatchQueue(label: "updateStringQueue")
                queue.async {
                    while true {
                        DispatchQueue.main.async {
                            self.elapsedTime = stopwatch.elapsedTime.formatTimeString
                        }
                        Thread.sleep(forTimeInterval: 0.03)
                    }
                }
            }
            
            LapTimeTable(laptimes: stopwatch.lapTimes)
            
            
            HStack(content: {
                if stopwatch.isRunning {
                    Button(action: {
                        stopwatch.addLap()
                    }) {
                        Text("Lap").padding()
                    }
                    Button(action: {
                        stopwatch.stop()
                    }) {
                        Text("Stop").padding()
                    }
                } else {
                    if elapsedTime != "00:00.00" {
                        Button(action: {
                            stopwatch.reset()
                        }) {
                            Text("Reset").padding()
                        }
                    }
                    
                    Button(action: {
                        stopwatch.start()
                    }) {
                        Text("Start").padding()
                    }
                }
                
                
            })
        }
        .padding()
    }
}

struct StopwatchView_Previews: PreviewProvider {
    static var previews: some View {
        StopwatchView()
    }
}
