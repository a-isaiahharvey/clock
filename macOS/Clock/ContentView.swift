//
//  ContentView.swift
//  Clock
//
//  Created by Allister Harvey on 3/8/23.
//

import SwiftUI
import UserNotifications

struct AppState {
  var stopwatch: Stopwatch?
  var timer: RTimer?
}

struct ContentView: View {
  @Binding var tabSelection: Int

  var body: some View {
    VStack {
      TabView(selection: $tabSelection) {
        StopwatchView()
          .tabItem {
            Text("Stopwatch")
          }.tag(3)
        RTimerView().tabItem {
          Text("Timer")
        }.tag(4)
      }
    }
    .padding()
    .onAppear {
      UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound, .badge]) {
        granted, error in
        if granted {
          print("Notification permission granted")
        } else {
          print("Notification permission denied")
        }
      }
    }.onDisappear {

    }
  }
}

struct ContentView_Previews: PreviewProvider {
  @State static var tabSelection = 3
  static var previews: some View {
    ContentView(tabSelection: $tabSelection)
  }
}
