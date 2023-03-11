//
//  ContentView.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import SwiftUI

struct ContentView: View {
    @State private var stopwatch: Stopwatch = Stopwatch()
    @State private var elapsedTime = "00:00.0"
    @State private var timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    
    
    var body: some View {
        VStack {
            TabView {
                StopwatchView()
                .tabItem {
                    Text("Stopwatch")
                }
            }
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
